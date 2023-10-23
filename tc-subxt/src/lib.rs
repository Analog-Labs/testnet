use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use subxt::tx::TxPayload;
use subxt::{tx::SubmittableExtrinsic, OnlineClient, PolkadotConfig};
use subxt_signer::{sr25519::Keypair, SecretUri};
use time_primitives::{AccountId, PublicKey};

#[subxt::subxt(
	runtime_metadata_path = "../config/subxt/metadata.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod timechain_runtime {}

mod members;
mod shards;
mod tasks;

pub use subxt::utils::H256;

#[derive(Clone)]
pub struct SubxtClient {
	client: Arc<OnlineClient<PolkadotConfig>>,
	signer: Arc<Keypair>,
	nonce: Arc<AtomicU64>,
}

impl SubxtClient {
	fn make_transaction<Call>(&self, call: &Call) -> Vec<u8>
	where
		Call: TxPayload,
	{
		let nonce = self.nonce.fetch_add(1, Ordering::SeqCst);
		tracing::info!("current nonce {}", nonce);
		self.client
			.tx()
			.create_signed_with_nonce(call, self.signer.as_ref(), nonce, Default::default())
			.unwrap()
			.into_encoded()
	}

	pub async fn new(keyfile: &Path) -> Result<Self> {
		let content = fs::read_to_string(keyfile).context("failed to read substrate keyfile")?;
		let secret = SecretUri::from_str(&content).context("failed to parse substrate keyfile")?;
		let keypair =
			Keypair::from_uri(&secret).context("substrate keyfile contains invalid suri")?;
		let account_id: subxt::utils::AccountId32 = keypair.public_key().into();
		let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944").await?;
		let nonce = api.tx().account_nonce(&account_id).await?;
		Ok(Self {
			client: Arc::new(api),
			signer: Arc::new(keypair),
			nonce: Arc::new(AtomicU64::new(nonce)),
		})
	}

	pub async fn submit_transaction(&self, transaction: Vec<u8>) -> Result<H256> {
		let hash = SubmittableExtrinsic::from_bytes((*self.client).clone(), transaction)
			.submit()
			.await?;
		Ok(hash)
	}

	pub fn public_key(&self) -> PublicKey {
		let public_key = self.signer.public_key();
		PublicKey::Sr25519(unsafe { std::mem::transmute(public_key) })
	}

	pub fn account_id(&self) -> AccountId {
		let account_id: subxt::utils::AccountId32 = self.signer.public_key().into();
		unsafe { std::mem::transmute(account_id) }
	}
}
