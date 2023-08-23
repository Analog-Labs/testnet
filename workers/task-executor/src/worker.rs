use crate::{TaskExecutorParams, TW_LOG};
use anyhow::{anyhow, Context, Result};
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, SinkExt, StreamExt};
use rosetta_client::{create_wallet, types::PartialBlockIdentifier, EthereumExt, Wallet};
use sc_client_api::{Backend, BlockchainEvents};
use sp_api::{HeaderT, ProvideRuntimeApi};
use sp_runtime::traits::Block;
use std::{collections::HashSet, future::Future, marker::PhantomData, pin::Pin, sync::Arc};
use time_primitives::{
	CycleStatus, Function, OcwPayload, PeerId, ShardId, TaskCycle, TaskError, TaskExecution,
	TaskId, TaskSpawner, TimeApi, TssId, TssRequest, TssSignature,
};
use timegraph_client::{Timegraph, TimegraphData};

pub struct TaskSpawnerParams {
	pub tss: mpsc::Sender<TssRequest>,
	pub connector_url: Option<String>,
	pub connector_blockchain: Option<String>,
	pub connector_network: Option<String>,
	pub timegraph_url: Option<String>,
	pub timegraph_ssk: Option<String>,
}

#[derive(Clone)]
pub struct Task {
	tss: mpsc::Sender<TssRequest>,
	wallet: Arc<Wallet>,
	timegraph: Option<Arc<Timegraph>>,
}

impl Task {
	pub async fn new(params: TaskSpawnerParams) -> Result<Self> {
		let wallet = Arc::new(
			create_wallet(
				params.connector_blockchain,
				params.connector_network,
				params.connector_url,
				None,
			)
			.await?,
		);
		let timegraph = if let Some(url) = params.timegraph_url {
			Some(Arc::new(Timegraph::new(
				url,
				params
					.timegraph_ssk
					.as_deref()
					.ok_or(anyhow!("timegraph session key is not specified"))?
					.to_string(),
			)?))
		} else {
			None
		};
		Ok(Self {
			tss: params.tss,
			wallet,
			timegraph,
		})
	}

	async fn execute_function(
		&self,
		function: &Function,
		target_block_number: u64,
	) -> Result<String> {
		let block = PartialBlockIdentifier {
			index: Some(target_block_number),
			hash: None,
		};
		Ok(match function {
			Function::EvmViewCall {
				address,
				function_signature,
				input,
			} => {
				let data = self
					.wallet
					.eth_view_call(address, function_signature, input, Some(block))
					.await?;
				serde_json::to_string(&data.result)?
			},
			Function::EvmTxReceipt { tx } => {
				let data = self.wallet.eth_transaction_receipt(tx).await?;
				serde_json::to_string(&data.result)?
			},
			Function::EvmDeploy { bytecode } => {
				self.wallet.eth_deploy_contract(bytecode.clone()).await?.hash
			},
			Function::EvmCall {
				address,
				function_signature,
				input,
				amount,
			} => {
				self.wallet
					.eth_send_call(address, function_signature, input, *amount)
					.await?
					.hash
			},
		})
	}

	async fn tss_sign(
		&self,
		block_number: u64,
		shard_id: ShardId,
		task_id: TaskId,
		cycle: TaskCycle,
		result: &str,
	) -> Result<TssSignature> {
		let data = bincode::serialize(&result).context("Failed to serialize task")?;
		let (tx, rx) = oneshot::channel();
		self.tss
			.clone()
			.send(TssRequest {
				request_id: TssId(task_id, cycle),
				shard_id,
				block_number,
				data,
				tx,
			})
			.await?;
		Ok(rx.await?)
	}

	async fn execute_read(
		self,
		target_block: u64,
		shard_id: ShardId,
		task_id: TaskId,
		task_cycle: TaskCycle,
		function: Function,
		hash: String,
		block_num: u64,
	) -> Result<TssSignature> {
		let result = self
			.execute_function(&task.function, target_block)
			.await
			.with_context(|| format!("Failed to execute {:?}", task.function))?;
		let signature = self
			.tss_sign(block_num, shard_id, task_id, task_cycle, &result)
			.await
			.with_context(|| format!("Failed to tss sign on shard {}", shard_id))?;
		if let Some(timegraph) = self.timegraph.as_ref() {
			timegraph
				.submit_data(TimegraphData {
					collection: hash,
					task_id,
					task_cycle,
					target_block_number: target_block,
					timechain_block_number: block_num,
					shard_id,
					signature,
					data: vec![result],
				})
				.await
				.context("Failed to submit data to timegraph")?;
		}
		Ok(signature)
	}

	async fn execute_write(self, function: Function) -> Result<String> {
		self.execute_function(&function, 0).await
	}
}

#[async_trait::async_trait]
impl TaskSpawner for Task {
	async fn block_height(&self) -> Result<u64> {
		let status = self.wallet.status().await?;
		Ok(status.index)
	}

	fn execute_read(
		&self,
		target_block: u64,
		shard_id: ShardId,
		task_id: TaskId,
		cycle: TaskCycle,
		function: Function,
		hash: String,
		block_num: u64,
	) -> Pin<Box<dyn Future<Output = Result<TssSignature>> + Send + 'static>> {
		self.clone()
			.execute(target_block, shard_id, task_id, cycle, task, block_num)
			.map(move |res| res.with_context(|| format!("Task {}/{} failed", task_id, cycle)))
			.boxed()
	}

	fn execute_write(
		&self,
		function: Function,
	) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'static>> {
		self.clone().execute_write(function).boxed()
	}
}

pub struct TaskExecutor<B: Block, BE, C, R, T> {
	_block: PhantomData<B>,
	backend: Arc<BE>,
	client: Arc<C>,
	runtime: Arc<R>,
	peer_id: PeerId,
	running_tasks: HashSet<TaskExecution>,
	task_spawner: T,
}

impl<B, BE, C, R, T> TaskExecutor<B, BE, C, R, T>
where
	B: Block,
	BE: Backend<B> + 'static,
	C: BlockchainEvents<B>,
	R: ProvideRuntimeApi<B>,
	R::Api: TimeApi<B>,
	T: TaskSpawner,
{
	pub fn new(params: TaskExecutorParams<B, BE, C, R, T>) -> Self {
		let TaskExecutorParams {
			_block,
			backend,
			client,
			runtime,
			peer_id,
			task_spawner,
		} = params;
		Self {
			_block,
			backend,
			client,
			runtime,
			peer_id,
			running_tasks: Default::default(),
			task_spawner,
		}
	}

	pub async fn start_tasks(
		&mut self,
		block_hash: <B as Block>::Hash,
		block_num: u64,
	) -> Result<()> {
		let block_height =
			self.task_spawner.block_height().await.context("Failed to fetch block height")?;
		let shards = self.runtime.runtime_api().get_shards(block_hash, self.peer_id)?;
		for shard_id in shards {
			let tasks = self.runtime.runtime_api().get_shard_tasks(block_hash, shard_id)?;
			log::info!("got task ====== {:?}", tasks);
			for executable_task in tasks {
				let task_id = executable_task.task_id;
				let cycle = executable_task.cycle;
				let retry_count = executable_task.retry_count;
				if self.running_tasks.contains(&executable_task) {
					continue;
				}
				let task_descr = self.runtime.runtime_api().get_task(block_hash, task_id)?.unwrap();
				let target_block_number = task_descr.trigger(cycle);
				let function = task_descr.function;
				let hash = task_descr.hash;
				if block_height >= target_block_number {
					log::info!("Running Task {}", executable_task);
					self.running_tasks.insert(executable_task.clone());
					let storage = self.backend.offchain_storage().unwrap();
					if executable_task.phase.is_write() {
						let task = self.task_spawner.execute_write(function);
						tokio::task::spawn(async move {
							let result = task.await.map_err(|e| e.to_string());
							log::info!(
								"Task {}/{}/{} completed with {:?}",
								task_id,
								cycle,
								retry_count,
								result
							);
							match result {
								Ok(hash) => {
									time_primitives::write_message(
										storage,
										&OcwPayload::SubmitTaskHash { shard_id, task_id, hash },
									);
								},
								Err(error) => {
									let error = TaskError { shard_id, error };
									time_primitives::write_message(
										storage,
										&OcwPayload::SubmitTaskError { task_id, error },
									);
								},
							}
						});
					} else {
						let function = if let Some(tx) = executable_task.phase.tx_hash() {
							Function::EvmTxReceipt { tx: tx.to_string() }
						} else {
							function
						};
						let task = self.task_spawner.execute_read(
							target_block_number,
							shard_id,
							task_id,
							cycle,
							function,
							hash,
							block_num,
						);
						tokio::task::spawn(async move {
							let result = task.await.map_err(|e| e.to_string());
							log::info!(
								"Task {}/{}/{} completed with {:?}",
								task_id,
								cycle,
								retry_count,
								result
							);
							match result {
								Ok(signature) => {
									let status = CycleStatus { shard_id, signature };
									time_primitives::write_message(
										storage,
										&OcwPayload::SubmitTaskResult { task_id, cycle, status },
									);
								},
								Err(error) => {
									let error = TaskError { shard_id, error };
									time_primitives::write_message(
										storage,
										&OcwPayload::SubmitTaskError { task_id, error },
									);
								},
							}
						});
					}
				}
			}
		}
		Ok(())
	}

	pub async fn run(&mut self) {
		let mut finality_notifications = self.client.finality_notification_stream();
		while let Some(notification) = finality_notifications.next().await {
			let block_hash = notification.header.hash();
			let block_num = notification.header.number().to_string().parse().unwrap();
			log::debug!(target: TW_LOG, "finalized {}", notification.header.number());
			if let Err(err) = self.start_tasks(block_hash, block_num).await {
				log::error!(target: TW_LOG, "error processing tasks: {}", err);
			}
		}
	}
}
