use crate::worker::TaskExecutor;
use futures::channel::mpsc::Sender;
use sc_client_api::Backend;
use sp_api::ProvideRuntimeApi;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::Block;
use std::{marker::PhantomData, sync::Arc};
use time_primitives::TimeApi;

// mod task_schedule;
mod worker;

#[cfg(test)]
mod tests;

/// Constant to indicate target for logging
pub const TW_LOG: &str = "task-executor";
pub type BlockHeight = u64;
/// Set of properties we need to run our gadget
#[derive(Clone)]
pub struct TaskExecutorParams<B: Block, A, BN, R, BE>
where
	B: Block,
	A: codec::Codec + Clone,
	BN: codec::Codec + Clone,
	BE: Backend<B>,
	R: ProvideRuntimeApi<B>,
	R::Api: TimeApi<B, A, BN>,
{
	pub backend: Arc<BE>,
	pub runtime: Arc<R>,
	pub kv: KeystorePtr,
	pub _block: PhantomData<B>,
	pub account_id: PhantomData<A>,
	pub _block_number: PhantomData<BN>,
	pub sign_data_sender: Sender<(u64, u64, u64, [u8; 32])>,
	pub connector_url: Option<String>,
	pub connector_blockchain: Option<String>,
	pub connector_network: Option<String>,
}

/// Start the task Executor gadget.
///
/// This is a thin shim around running and awaiting a task Executor.
pub async fn start_task_executor_gadget<B, A, BN, R, BE>(
	params: TaskExecutorParams<B, A, BN, R, BE>,
	repetitive: bool,
) where
	B: Block,
	A: codec::Codec + Clone + 'static,
	BN: codec::Codec + Clone + 'static,
	R: ProvideRuntimeApi<B>,
	BE: Backend<B>,
	R::Api: TimeApi<B, A, BN>,
{
	log::debug!(target: TW_LOG, "Starting task-executor gadget");
	let mut worker = TaskExecutor::new(params).await.unwrap();
	if repetitive {
		worker.run_repetitive_task().await;
	} else {
		worker.run().await;
	}
}
