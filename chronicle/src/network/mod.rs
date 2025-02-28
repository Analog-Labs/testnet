use self::protocol::TssEndpoint;
use anyhow::Result;
use futures::channel::mpsc;
use futures::stream::BoxStream;
use futures::{Future, StreamExt};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use time_primitives::{BlockNumber, ShardId, TaskId};

mod protocol;

pub use time_primitives::PeerId;

pub type TssMessage = tss::TssMessage<TaskId>;

pub const PROTOCOL_NAME: &str = "/analog-labs/chronicle/1";

#[derive(Default)]
pub struct NetworkConfig {
	pub secret: [u8; 32],
}

#[derive(Deserialize, Serialize)]
pub struct Message {
	pub shard_id: ShardId,
	pub block: BlockNumber,
	pub payload: TssMessage,
}

pub trait Network: Send + Sync + 'static {
	fn peer_id(&self) -> PeerId;

	fn format_peer_id(&self, peer_id: PeerId) -> String;

	fn send(
		&self,
		peer_id: PeerId,
		msg: Message,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}

impl Network for Arc<dyn Network> {
	fn peer_id(&self) -> PeerId {
		self.deref().peer_id()
	}

	fn format_peer_id(&self, peer: PeerId) -> String {
		self.deref().format_peer_id(peer)
	}

	fn send(
		&self,
		peer_id: PeerId,
		msg: Message,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
		self.deref().send(peer_id, msg)
	}
}

pub async fn create_iroh_network(
	config: NetworkConfig,
) -> Result<(Arc<dyn Network>, BoxStream<'static, (PeerId, Message)>)> {
	let (net_tx, net_rx) = mpsc::channel(10);
	let network =
		Arc::new(TssEndpoint::new(config, net_tx).await?) as Arc<dyn Network + Send + Sync>;
	let incoming = net_rx.boxed();
	Ok((network, incoming))
}
