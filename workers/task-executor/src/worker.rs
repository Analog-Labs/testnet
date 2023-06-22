use crate::{TaskExecutorParams, TW_LOG};
use anyhow::{Context, Result};
use codec::{Decode, Encode};
use futures::channel::mpsc::Sender;
use rosetta_client::{
	create_client,
	types::{CallRequest, CallResponse},
	BlockchainConfig, Client,
};
use sc_client_api::Backend;
use serde_json::json;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::Backend as _;
use sp_core::{hashing::keccak_256, offchain::STORAGE_PREFIX};
use sp_keystore::KeystorePtr;
use sp_runtime::offchain::OffchainStorage;
use sp_runtime::traits::Block;
use std::{
	collections::{BTreeMap, HashSet, VecDeque},
	marker::PhantomData,
	sync::Arc,
	time::Duration,
};
use time_primitives::{
	abstraction::{Function, OCWSkdData, ScheduleStatus},
	KeyId, TaskSchedule, TimeApi, TimeId, OCW_SKD_KEY, TIME_KEY_TYPE,
};

#[derive(Clone)]
pub struct TaskExecutor<B, BE, R, A> {
	_block: PhantomData<B>,
	backend: Arc<BE>,
	runtime: Arc<R>,
	_account_id: PhantomData<A>,
	sign_data_sender: Sender<(u64, u64, u64, [u8; 32])>,
	kv: KeystorePtr,
	// all tasks that are scheduled
	// TODO need to get all completed task and remove them from it
	tasks: HashSet<u64>,
	rosetta_chain_config: BlockchainConfig,
	rosetta_client: Client,
}

impl<B, BE, R, A> TaskExecutor<B, BE, R, A>
where
	B: Block,
	BE: Backend<B> + 'static,
	R: ProvideRuntimeApi<B> + 'static,
	A: codec::Codec + Clone + 'static,
	R::Api: TimeApi<B, A>,
{
	pub async fn new(params: TaskExecutorParams<B, A, R, BE>) -> Result<Self> {
		let TaskExecutorParams {
			backend,
			runtime,
			sign_data_sender,
			kv,
			_block,
			account_id: _,
			connector_url,
			connector_blockchain,
			connector_network,
		} = params;

		// create rosetta client and get chain configuration
		let (rosetta_chain_config, rosetta_client) =
			create_client(connector_blockchain, connector_network, connector_url).await?;

		Ok(Self {
			_block: PhantomData,
			backend,
			runtime,
			_account_id: PhantomData,
			sign_data_sender,
			kv,
			tasks: Default::default(),
			rosetta_chain_config,
			rosetta_client,
		})
	}

	fn account_id(&self) -> Option<TimeId> {
		let keys = self.kv.sr25519_public_keys(TIME_KEY_TYPE);
		if keys.is_empty() {
			log::warn!(target: TW_LOG, "No time key found, please inject one.");
			None
		} else {
			let id = &keys[0];
			TimeId::decode(&mut id.as_ref()).ok()
		}
	}

	/// Encode call response and send data for tss signing process
	async fn send_for_sign(
		&mut self,
		block_id: <B as Block>::Hash,
		data: CallResponse,
		shard_id: u64,
		schedule_id: u64,
		schedule_cycle: u64,
	) -> Result<bool> {
		let bytes = bincode::serialize(&data.result).context("Failed to serialize task")?;
		let hash = keccak_256(&bytes);

		self.sign_data_sender
			.clone()
			.try_send((shard_id, schedule_id, schedule_cycle, hash))?;
		self.tasks.insert(schedule_id);

		if self.is_collector(block_id, shard_id).unwrap_or(false) {
			self.update_schedule_ocw_storage(ScheduleStatus::Completed, schedule_id);
		}

		Ok(true)
	}

	/// Fetches and executes contract call for a given schedule_id
	async fn task_executor(
		&mut self,
		block_id: <B as Block>::Hash,
		schedule_id: &u64,
		schedule: &TaskSchedule<A>,
	) -> Result<()> {
			let metadata = self
				.runtime
				.runtime_api()
				.get_task_metadata_by_key(block_id, schedule.task_id.0)?
				.map_err(|err| anyhow::anyhow!("{:?}", err))?;

			let shard_id = schedule.shard_id;
			let Some(task) = metadata else {
					log::info!("task schedule id have no metadata, Removing task from Schedule list");

					if self.is_collector(block_id, shard_id).unwrap_or(false) {
						self.update_schedule_ocw_storage(ScheduleStatus::Invalid, *schedule_id);
					}

					return Ok(());
				};

			match &task.function {
				// If the task function is an Ethereum contract
				// call, call it and send for signing
				Function::EthereumViewWithoutAbi {
					address,
					function_signature,
					input: _,
					output: _,
				} => {
					log::info!("running task_id {:?}", schedule_id);
					let method = format!("{address}-{function_signature}-call");
					let request = CallRequest {
						network_identifier: self.rosetta_chain_config.network(),
						method,
						parameters: json!({}),
					};
					let data = self.rosetta_client.call(&request).await?;
					if !self
						.send_for_sign(
							block_id,
							data.clone(),
							shard_id,
							*schedule_id,
							schedule.cycle,
						)
						.await?
					{
						log::warn!("status not updated can't updated data into DB");
						return Ok(());
					}
				},
				_ => {
					log::warn!("error on matching task function")
				},
			};
		Ok(())
	}

	/// check if current node is collector
	fn is_collector(&self, block_id: <B as Block>::Hash, shard_id: u64) -> Result<bool> {
		let Some(account) = self.account_id() else {
			return Ok(false);
		};

		let available_shards = self.runtime.runtime_api().get_shards(block_id).unwrap_or(vec![]);
		if available_shards.is_empty() {
			anyhow::bail!("No shards available");
		}
		let Some(shard) = available_shards
							.into_iter()
							.find(|(s, _)| *s == shard_id)
							.map(|(_, s)| s) else {
			anyhow::bail!("failed to find shard");
		};

		Ok(*shard.collector() == account)
	}

	// entry point for task execution, triggered by each finalized block in the Timechain
	async fn process_tasks_for_block(&mut self, block_id: <B as Block>::Hash) -> Result<()> {
		let task_schedules = self
			.runtime
			.runtime_api()
			.get_one_time_task_schedule(block_id)?
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;
		log::info!("\n\n task schedule {:?}\n", task_schedules.len());

		let mut tree_map = BTreeMap::new();
		for (id, schedule) in task_schedules {
			// if task is already executed then skip
			if self.tasks.contains(&id) {
				continue;
			}
			tree_map.insert(id, schedule);
		}

		for (id, schedule) in tree_map.iter() {
			match self.task_executor(block_id, id, schedule).await {
				Ok(()) => self.update_schedule_ocw_storage(ScheduleStatus::Completed, *id),
				Err(e) => log::warn!("error in single task schedule result {:?}", e),
			}
		}

		Ok(())
	}

	/// Add schedule update task to offchain storage
	/// which will be use by offchain worker to send extrinsic
	fn update_schedule_ocw_storage(&mut self, schedule_status: ScheduleStatus, key: KeyId) {
		let ocw_skd = OCWSkdData::new(schedule_status, key);

		if let Some(mut ocw_storage) = self.backend.offchain_storage() {
			let old_value = ocw_storage.get(STORAGE_PREFIX, OCW_SKD_KEY);

			let mut ocw_vec = match old_value.clone() {
				Some(mut data) => {
					//remove this unwrap
					let mut bytes: &[u8] = &mut data;
					let inner_data: VecDeque<OCWSkdData> = Decode::decode(&mut bytes).unwrap();
					inner_data
				},
				None => Default::default(),
			};

			ocw_vec.push_back(ocw_skd);
			let encoded_data = Encode::encode(&ocw_vec);
			let is_data_stored = ocw_storage.compare_and_set(
				STORAGE_PREFIX,
				OCW_SKD_KEY,
				old_value.as_deref(),
				&encoded_data,
			);
			log::info!("stored task data in ocw {:?}", is_data_stored);
		} else {
			log::error!("cant get offchain storage");
		};
	}

	pub async fn run(&mut self) {
		loop {
			match self.backend.blockchain().last_finalized() {
				Ok(at) => {
					if let Err(e) = self.process_tasks_for_block(at).await {
						log::error!("Failed to process tasks for block {:?}: {:?}", at, e);
					}
				},
				Err(e) => {
					log::error!("Blockchain is empty: {}", e);
				},
			};
			tokio::time::sleep(Duration::from_secs(10)).await;
		}
	}
}
