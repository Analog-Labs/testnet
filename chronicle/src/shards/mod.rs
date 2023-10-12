use self::protocol::TssEndpoint;
use self::substrate::{SubstrateNetwork, SubstrateNetworkAdapter};
use anyhow::Result;
use futures::channel::mpsc;
use futures::stream::BoxStream;
use futures::{Future, StreamExt};
use sc_network::request_responses::IncomingRequest;
use sc_network::{NetworkRequest, NetworkSigner};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use time_primitives::{BlockNumber, ShardId};
use tss::TssMessage;

mod protocol;
mod service;
mod substrate;
#[cfg(test)]
mod tests;
mod tss;

pub use self::service::{TimeWorker, TimeWorkerParams};
pub use self::substrate::protocol_config;
pub use time_primitives::PeerId;

pub const PROTOCOL_NAME: &str = "analog-labs/chronicle/1";

#[derive(Default)]
pub struct NetworkConfig {
	pub secret: Option<[u8; 32]>,
	pub relay: Option<String>,
	pub bind_port: Option<u16>,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
	pub shard_id: ShardId,
	pub block_number: BlockNumber,
	pub payload: TssMessage,
}

pub trait Network: Send + Sync + 'static {
	fn peer_id(&self) -> PeerId;

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

	fn send(
		&self,
		peer_id: PeerId,
		msg: Message,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
		self.deref().send(peer_id, msg)
	}
}

pub async fn network<N: NetworkRequest + NetworkSigner + Send + Sync + 'static>(
	substrate: Option<(N, async_channel::Receiver<IncomingRequest>)>,
	config: NetworkConfig,
) -> Result<(Arc<dyn Network>, BoxStream<'static, (PeerId, Message)>)> {
	Ok(if let Some((network, incoming)) = substrate {
		let network = Arc::new(SubstrateNetwork::new(network)?) as Arc<dyn Network + Send + Sync>;
		let incoming = SubstrateNetworkAdapter::new(incoming).boxed();
		(network, incoming)
	} else {
		let (net_tx, net_rx) = mpsc::channel(10);
		let network =
			Arc::new(TssEndpoint::new(config, net_tx).await?) as Arc<dyn Network + Send + Sync>;
		let incoming = net_rx.boxed();
		(network, incoming)
	})
}
