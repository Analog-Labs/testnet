#![allow(clippy::type_complexity)]
use crate::WorkerParams;
use bincode::serialize;
use dotenvy::dotenv;
use futures::channel::mpsc::Sender;
use ink::env::hash;
use rosetta_client::{Client, BlockchainConfig, create_client, types::CallRequest};
use sc_client_api::Backend;
use serde_json::json;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::Backend as SpBackend;

use sp_runtime::{generic::BlockId, traits::Block};
use std::{collections::HashMap, error::Error, marker::PhantomData, sync::Arc, thread};
use time_primitives::{abstraction::Function, TimeApi};
use time_worker::kv::TimeKeyvault;
use tokio::{sync::Mutex, time};

// use worker_aurora::{self, establish_connection, get_on_chain_data};

#[allow(unused)]
/// Our structure, which holds refs to everything we need to operate
pub struct TaskExecutor<B: Block, R, BE> {
	pub(crate) backend: Arc<BE>,
	pub(crate) runtime: Arc<R>,
	_block: PhantomData<B>,
	sign_data_sender: Arc<Mutex<Sender<(u64, [u8; 32])>>>,
	kv: TimeKeyvault,
}

impl<B, R, BE> TaskExecutor<B, R, BE>
where
	B: Block,
	R: ProvideRuntimeApi<B>,
	BE: Backend<B>,
	R::Api: TimeApi<B>,
{
	pub(crate) fn new(worker_params: WorkerParams<B, R, BE>) -> Self {
		let WorkerParams {
			backend,
			runtime,
			sign_data_sender,
			kv,
			_block,
		} = worker_params;

		TaskExecutor {
			backend,
			runtime,
			sign_data_sender,
			kv,
			_block: PhantomData,
		}
	}

	pub fn hash_keccak_256(input: &[u8]) -> [u8; 32] {
		let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
		ink::env::hash_bytes::<hash::Keccak256>(input, &mut output);
		output
	}

	async fn call_contract_function(
		&self,
		address: String,
		function_signature: String,
	) -> Result<(), Box<dyn Error>> {
		dotenv().ok();

		let (config, client) = if let Ok(client_config) = create_connector_client().await {
			(client_config.0, client_config.1)
		} else {
			return Err("Failed to create connector client".into());
		};

		let method = format!("{}-{}-call", address, function_signature);

		let request = CallRequest {
			network_identifier: config.network(),
			method,
			parameters: json!({}),
		};

		let data = client.call(&request).await?;

		if let Ok(task_in_bytes) = serialize(&data.result) {
			println!("received data: {:?}", data.result);
			let hash = Self::hash_keccak_256(&task_in_bytes);
			match self.sign_data_sender.lock().await.try_send((1, hash)) {
				Ok(()) => {
					log::info!("Connector successfully send event to channel")
				},
				Err(_) => {
					log::info!("Connector failed to send event to channel")
				},
			}
		} else {
			log::info!("Failed to serialize task: {:?}", data);
		}
		Ok(())
	}

	pub(crate) async fn run(&mut self) {
		let delay = time::Duration::from_secs(10);
		let mut map: HashMap<u64, String> = HashMap::new();


		loop {
			let keys = self.kv.public_keys();
			if !keys.is_empty() {
				if let Ok(at) = self.backend.blockchain().last_finalized() {
					let at = BlockId::Hash(at);

					if let Ok(tasks_schedule) = self.runtime.runtime_api().get_task_schedule(&at) {
						match tasks_schedule {
							Ok(task_schedule) => {
								for schedule_task in task_schedule.iter() {
									let task_id = schedule_task.task_id.0;

									match map.insert(task_id, "hash".to_string()) {
										Some(old_value) => println!(
											"The key already existed with the value {}",
											old_value
										),
										None => println!(
											"The key didn't exist and was inserted key {}.",
											task_id
										),
									}

									if let Ok(metadata_result) = self
										.runtime
										.runtime_api()
										.get_task_metadat_by_key(&at, task_id)
									{
										match metadata_result {
											Ok(metadata) => {
												for task in metadata.iter() {
													match &task.function {
														Function::EthereumContractWithoutAbi {
															address,
															function_signature,
															input: _,
															output: _,
														} => {
															if let Err(e) = self
																.call_contract_function(
																	address.to_string(),
																	function_signature.to_string(),
																)
																.await
															{
																log::error!("Failed to call contract function: {:?}", e);
															}
														},
														_ => {
															log::warn!("Unsupported function type: {:?}", task.function)
														},
													};
												}
											},
											Err(e) => {
												log::info!(
													"No metadata found for block {:?}: {:?}",
													at,
													e
												);
											},
										}
									} else {
										log::error!(
											"Failed to get task metadata for block {:?}",
											at
										);
									}
								}
							},
							Err(e) => {
								log::info!("No metadata found for block {:?}: {:?}", at, e);
							},
						}
					} else {
						log::error!("Failed to get task schedule for block {:?}", at);
					}
				} else {
					log::error!("Blockchain is empty");
				}
				thread::sleep(delay);
			}
		}
	}
}

async fn create_connector_client() -> Result<(BlockchainConfig, Client), Box<dyn Error>> {
	let connector_url = std::env::var("CONNECTOR_URL").expect("CONNECTOR_URL must be set");
	let connector_blockchain =
		std::env::var("CONNECTOR_BLOCKCHAIN").expect("CONNECTOR_BLOCKCHAIN must be set");
	let connector_network =
		std::env::var("CONNECTOR_NETWORK").expect("CONNECTOR_NETWORK must be set");

	let (config, client) =
		create_client(Some(connector_blockchain), Some(connector_network), Some(connector_url))
			.await?;

	Ok((config, client))
}
