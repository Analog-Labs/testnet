use crate::worker::TaskExecutor;
use futures::channel::mpsc::Sender;
use sc_client_api::{Backend, BlockchainEvents};
use sp_api::ProvideRuntimeApi;
use sp_runtime::traits::Block;
use std::{marker::PhantomData, sync::Arc};
use time_primitives::{PeerId, TimeApi};
use time_worker::TssRequest;

mod worker;

//#[cfg(test)]
//mod tests;

/// Constant to indicate target for logging
pub const TW_LOG: &str = "task-executor";

/// Set of properties we need to run our gadget
#[derive(Clone)]
pub struct TaskExecutorParams<B: Block, BE, R>
where
	B: Block,
	BE: Backend<B> + 'static,
	R: BlockchainEvents<B> + ProvideRuntimeApi<B>,
	R::Api: TimeApi<B>,
{
	pub _block: PhantomData<B>,
	pub backend: Arc<BE>,
	pub runtime: Arc<R>,
	pub peer_id: PeerId,
	pub sign_data_sender: Sender<TssRequest>,
	pub connector_url: Option<String>,
	pub connector_blockchain: Option<String>,
	pub connector_network: Option<String>,
}

/// Start the task Executor gadget.
///
/// This is a thin shim around running and awaiting a task Executor.
pub async fn start_task_executor_gadget<B, BE, R>(params: TaskExecutorParams<B, BE, R>)
where
	B: Block,
	BE: Backend<B> + 'static,
	R: BlockchainEvents<B> + ProvideRuntimeApi<B>,
	R::Api: TimeApi<B>,
{
	log::debug!(target: TW_LOG, "Starting task-executor gadget");
	let mut worker = TaskExecutor::new(params).await.unwrap();
	worker.run().await;
}
