use anyhow::Result;
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use subxt::tx::Payload;
use subxt::utils::H256;
pub use subxt_signer::sr25519::Keypair;

use crate::{metadata, CommitteeEvent, ExtrinsicParams, OnlineClient, SubmittableExtrinsic};

pub struct BlockDetail {
	pub number: u64,
	pub hash: H256,
}

#[async_trait::async_trait]
pub trait ITimechainClient {
	type Submitter: ITransactionSubmitter + Send + Sync;
	type Block: IBlock + Send + Sync;
	async fn get_latest_block(&self) -> Result<BlockDetail>;
	fn sign_payload<Call>(&self, call: &Call, params: ExtrinsicParams) -> Vec<u8>
	where
		Call: Payload + Send + Sync;
	fn submittable_transaction(&self, tx: Vec<u8>) -> Self::Submitter;
	async fn finalized_block_stream(
		&self,
	) -> Result<BoxStream<'static, Result<(Self::Block, Vec<<Self::Block as IBlock>::Extrinsic>)>>>;
	async fn best_block_stream(
		&self,
	) -> Result<BoxStream<'static, Result<(Self::Block, Vec<<Self::Block as IBlock>::Extrinsic>)>>>;
}

#[async_trait::async_trait]
pub trait ITransactionSubmitter: Send + Sync {
	fn hash(&self) -> H256;
	async fn submit(&self) -> Result<H256>;
}

#[async_trait::async_trait]
pub trait IBlock: Send + Sync {
	type Extrinsic: IExtrinsic + Send + Sync;
	async fn extrinsics(&self) -> Result<Vec<Self::Extrinsic>>;
	fn number(&self) -> u64;
	fn hash(&self) -> H256;
}

#[async_trait::async_trait]
pub trait IExtrinsic: Send + Sync {
	type Events: Send + Sync;
	async fn events(&self) -> Result<Self::Events>;
	fn hash(&self) -> H256;
	async fn is_success(&self) -> Result<()>;
}

pub struct TimechainOnlineClient {
	client: OnlineClient,
	keypair: Keypair,
}

impl TimechainOnlineClient {
	pub fn new(client: OnlineClient, keypair: Keypair) -> Self {
		Self { client, keypair }
	}
}
pub struct SignedTransaction {
	tx: SubmittableExtrinsic,
}

pub struct TimechainBlock {
	pub block: crate::Block,
}

pub struct TimechainExtrinsic {
	pub extrinsic: crate::ExtrinsicDetails,
}

pub struct TimechainEvents {
	pub events: crate::ExtrinsicEvents,
}

#[async_trait::async_trait]
impl ITimechainClient for TimechainOnlineClient {
	type Submitter = SignedTransaction;
	type Block = TimechainBlock;

	async fn get_latest_block(&self) -> Result<BlockDetail> {
		let block = self.client.blocks().at_latest().await?;
		Ok(BlockDetail {
			number: block.number().into(),
			hash: block.hash(),
		})
	}

	fn sign_payload<Call>(&self, call: &Call, params: ExtrinsicParams) -> Vec<u8>
	where
		Call: Payload + Send + Sync,
	{
		self.client
			.tx()
			.create_signed_offline(call, &self.keypair, params)
			.expect("Metadata is invalid")
			.into_encoded()
	}

	fn submittable_transaction(&self, tx: Vec<u8>) -> Self::Submitter {
		let tx = SubmittableExtrinsic::from_bytes(self.client.clone(), tx);
		SignedTransaction { tx }
	}
	async fn finalized_block_stream(
		&self,
	) -> Result<BoxStream<'static, Result<(Self::Block, Vec<<Self::Block as IBlock>::Extrinsic>)>>>
	{
		let finalized_stream = self.client.blocks().subscribe_finalized().await?;
		let stream_with_txs = finalized_stream.map(|res| res.map_err(anyhow::Error::new)).and_then(
			|block| async move {
				let block = TimechainBlock { block };
				let extrinsics = IBlock::extrinsics(&block).await?;
				Ok((block, extrinsics))
			},
		);
		Ok(stream_with_txs.boxed())
	}
	async fn best_block_stream(
		&self,
	) -> Result<BoxStream<'static, Result<(Self::Block, Vec<<Self::Block as IBlock>::Extrinsic>)>>>
	{
		let best_stream = self.client.blocks().subscribe_best().await?;
		let stream_with_txs =
			best_stream
				.map(|res| res.map_err(anyhow::Error::new))
				.and_then(|block| async move {
					let block = TimechainBlock { block };
					let extrinsics = IBlock::extrinsics(&block).await?;
					Ok((block, extrinsics))
				});
		Ok(stream_with_txs.boxed())
	}

	// async fn runtime_updates(&self) {
	// 	todo!()
	// 	let updater= self.client.updater();
	// 	let updates = updater.runtime_updates();
	// }
}

#[async_trait::async_trait]
impl ITransactionSubmitter for SignedTransaction {
	fn hash(&self) -> H256 {
		self.tx.hash()
	}
	async fn submit(&self) -> Result<H256> {
		self.tx.submit().await.map_err(|e| anyhow::anyhow!(e))
	}
}

#[async_trait::async_trait]
impl IBlock for TimechainBlock {
	type Extrinsic = TimechainExtrinsic;
	async fn extrinsics(&self) -> Result<Vec<Self::Extrinsic>> {
		let extrinsics = self.block.extrinsics().await?;
		Ok(extrinsics.iter().map(|extrinsic| TimechainExtrinsic { extrinsic }).collect())
	}
	fn number(&self) -> u64 {
		self.block.number().into()
	}
	fn hash(&self) -> H256 {
		self.block.hash()
	}
}

#[async_trait::async_trait]
impl IExtrinsic for TimechainExtrinsic {
	type Events = TimechainEvents;
	async fn events(&self) -> Result<Self::Events> {
		Ok(TimechainEvents {
			events: self.extrinsic.events().await?,
		})
	}
	fn hash(&self) -> H256 {
		self.extrinsic.hash()
	}
	async fn is_success(&self) -> Result<()> {
		type SpRuntimeDispatchError = metadata::runtime_types::sp_runtime::DispatchError;
		let events = self.extrinsic.events().await?;
		for ev in events.iter() {
			let ev = ev?;

			if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
				let event_metadata = ev.event_metadata();
				anyhow::bail!(
					"{:?} extrinsic failed with code: {:?}, pallet idx: {}, variant idx: {}",
					self.hash(),
					ev.field_bytes(),
					event_metadata.pallet.index(),
					event_metadata.variant.index,
				)
			}

			if let Some(event) = ev.as_event::<CommitteeEvent::MemberExecuted>()? {
				if let Err(err) = event.result {
					let SpRuntimeDispatchError::Module(error) = err else {
						anyhow::bail!("Tx failed with error: {:?}", err);
					};
					let event_metadata = ev.event_metadata();

					let Some(error_metadata) =
						event_metadata.pallet.error_variant_by_index(error.error[0])
					else {
						anyhow::bail!("Tx failed with error: {:?}", error);
					};

					anyhow::bail!("Tx failed with error: {:?}", error_metadata.name);
				}
			}
		}
		Ok(())
	}
}
