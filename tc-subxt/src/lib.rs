#![allow(clippy::missing_transmute_annotations)]
use crate::worker::{SubxtWorker, Tx};
use anyhow::{Context, Result};
use futures::channel::{mpsc, oneshot};
use futures::stream::BoxStream;
use std::future::Future;
use std::str::FromStr;
use std::time::Duration;
use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClient as Client};
use subxt::backend::rpc::RpcClient;
use subxt::config::DefaultExtrinsicParams;
use subxt::PolkadotConfig;
use subxt_signer::SecretUri;

use time_primitives::{AccountId, BlockHash, BlockNumber, PublicKey, H256};

mod api;
mod metadata;
mod worker;

use metadata::technical_committee::events as CommitteeEvent;

pub use subxt_signer::sr25519::Keypair;

pub type OnlineClient = subxt::OnlineClient<PolkadotConfig>;
pub type LegacyRpcMethods = subxt::backend::legacy::LegacyRpcMethods<subxt::PolkadotConfig>;
pub type ExtrinsicEvents = subxt::blocks::ExtrinsicEvents<PolkadotConfig>;
pub type ExtrinsicDetails = subxt::blocks::ExtrinsicDetails<PolkadotConfig, OnlineClient>;
pub type SubmittableExtrinsic = subxt::tx::SubmittableExtrinsic<PolkadotConfig, OnlineClient>;
pub type ExtrinsicParams =
	<DefaultExtrinsicParams<PolkadotConfig> as subxt::config::ExtrinsicParams<PolkadotConfig>>::Params;
pub type TxInBlock = subxt::tx::TxInBlock<PolkadotConfig, OnlineClient>;
pub type TxProgress = subxt::tx::TxProgress<PolkadotConfig, OnlineClient>;

#[derive(Clone)]
pub struct SubxtClient {
	client: OnlineClient,
	tx: mpsc::UnboundedSender<(Tx, oneshot::Sender<ExtrinsicDetails>)>,
	public_key: PublicKey,
	account_id: AccountId,
}

impl SubxtClient {
	pub async fn new(url: &str, keypair: Keypair) -> Result<Self> {
		let rpc = Self::get_client(url).await?;
		let client = OnlineClient::from_rpc_client(rpc.clone())
			.await
			.map_err(|_| anyhow::anyhow!("Failed to create a new client"))?;
		let worker = SubxtWorker::new(rpc, client.clone(), keypair).await?;
		let public_key = worker.public_key();
		let account_id = worker.account_id();
		tracing::info!("account id {}", account_id);
		let tx = worker.into_sender();
		Ok(Self {
			client,
			tx,
			public_key,
			account_id,
		})
	}

	pub async fn with_key(url: &str, mnemonic: &str) -> Result<Self> {
		let secret =
			SecretUri::from_str(mnemonic.trim()).context("failed to parse substrate keyfile")?;
		let keypair = Keypair::from_uri(&secret).context("substrate keyfile contains uri")?;
		Self::new(url, keypair).await
	}

	pub async fn get_client(url: &str) -> Result<RpcClient> {
		let client = Client::builder()
			.retry_policy(
				ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)).take(3),
			)
			.build(url.to_string())
			.await?;
		Ok(client.into())
	}

	pub fn public_key(&self) -> &PublicKey {
		&self.public_key
	}

	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	pub async fn latest_block(&self) -> Result<u64> {
		Ok(self.client.blocks().at_latest().await?.number().into())
	}

	pub fn block_notification_stream(&self) -> BoxStream<'static, (BlockHash, BlockNumber)> {
		let client = self.client.clone();
		let f = move || client.blocks().subscribe_all();
		block_stream(f)
	}

	pub fn finality_notification_stream(&self) -> BoxStream<'static, (BlockHash, BlockNumber)> {
		let client = self.client.clone();
		let f = move || client.blocks().subscribe_finalized();
		block_stream(f)
	}

	pub async fn set_code(&self, code: Vec<u8>) -> Result<()> {
		let (tx, rx) = oneshot::channel();
		self.tx.unbounded_send((Tx::SetCode { code }, tx))?;
		let tx = rx.await?;
		self.wait_for_success(tx).await?;
		Ok(())
	}

	pub async fn transfer(&self, account: AccountId, balance: u128) -> Result<()> {
		let (tx, rx) = oneshot::channel();
		self.tx.unbounded_send((Tx::Transfer { account, balance }, tx))?;
		let tx = rx.await?;
		self.wait_for_success(tx).await?;
		Ok(())
	}

	pub async fn balance(&self, account: &AccountId) -> Result<u128> {
		let storage_query =
			metadata::storage().system().account(subxt::utils::Static(account.clone()));
		let result = self.client.storage().at_latest().await?.fetch(&storage_query).await?;
		Ok(if let Some(info) = result { info.data.free } else { 0 })
	}

	pub async fn wait_for_success(&self, extrinsic: ExtrinsicDetails) -> Result<ExtrinsicEvents> {
		type SpRuntimeDispatchError = metadata::runtime_types::sp_runtime::DispatchError;
		let events = extrinsic.events().await?;

		for ev in events.iter() {
			let ev = ev?;

			if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
				let dispatch_error = subxt::error::DispatchError::decode_from(
					ev.field_bytes(),
					self.client.metadata(),
				)?;
				return Err(dispatch_error.into());
			}

			if let Some(event) = ev.as_event::<CommitteeEvent::MemberExecuted>()? {
				if let Err(err) = event.result {
					let SpRuntimeDispatchError::Module(error) = err else {
						anyhow::bail!("Tx failed with error: {:?}", err);
					};

					let metadata = self.client.metadata();
					let error_pallet = metadata
						.pallet_by_index(error.index)
						.ok_or_else(|| anyhow::anyhow!("Pallet not found: {:?}", error.index))?;

					let Some(error_metadata) = error_pallet.error_variant_by_index(error.error[0])
					else {
						anyhow::bail!("Tx failed with error: {:?}", error);
					};

					anyhow::bail!("Tx failed with error: {:?}", error_metadata.name);
				}
			}
		}

		Ok(events)
	}
}

type Block = subxt::blocks::Block<subxt::PolkadotConfig, OnlineClient>;
type BlockStreamOutput = Result<subxt::backend::StreamOfResults<Block>, subxt::error::Error>;

fn block_stream<
	B: Future<Output = BlockStreamOutput> + Send + 'static,
	F: Fn() -> B + Send + 'static,
>(
	f: F,
) -> BoxStream<'static, (BlockHash, BlockNumber)> {
	let stream = async_stream::stream! {
		loop {
			let mut block_stream = match f().await {
				Ok(stream) => stream,
				Err(e) => {
					tracing::error!("Error subscribing to block stream {:?}", e);
					tokio::time::sleep(Duration::from_secs(1)).await;
					continue;
				},
			};
			while let Some(block_result) = block_stream.next().await {
				match block_result {
					Ok(block) => {
						let block_hash = block.hash();
						let block_number = block.header().number;
						yield (H256(block_hash.0), block_number);
					},
					Err(e) => {
						if e.is_disconnected_will_reconnect() {
							tracing::error!("subxt connection was lost and we may have missed a few blocks");
							continue;
						}
						tracing::error!("Subxt error: {:?}", e);
						tokio::time::sleep(Duration::from_secs(1)).await;
						break;
					},
				}
			}
		}
	};
	Box::pin(stream)
}
