use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use subxt::tx::TxPayload;
use subxt::{
	backend::Backend, constants::ConstantsClient, tx::SubmittableExtrinsic, OnlineClient,
	PolkadotConfig,
};
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair, SecretUri};
#[subxt::subxt(
	runtime_metadata_path = "../config/subxt/metadata.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod timechain_runtime {}
mod members;
mod shards;
mod tasks;
pub type KeyPair = sp_core::sr25519::Pair;

pub struct SubxtClient {
	client: Arc<OnlineClient<PolkadotConfig>>,
	signer: Arc<Keypair>,
	nonce: u64,
}

impl SubxtClient {
	pub fn make_transaction<Call>(&mut self, call: &Call) -> Vec<u8>
	where
		Call: TxPayload,
	{
		let tx_bytes = self
			.client
			.tx()
			.create_signed_with_nonce(call, self.signer.as_ref(), self.nonce, Default::default())
			.unwrap()
			.into_encoded();
		self.nonce += 1;
		tx_bytes
	}

	pub async fn new(keyfile: String) -> Self {
		let content = fs::read_to_string(keyfile).expect("file path not found");
		let secret =
			SecretUri::from_str(&content).expect("cannot create secret from content of file");
		let keypair = Keypair::from_uri(&secret).expect("cannot create keypair from secret");
		let account_id: subxt::utils::AccountId32 = keypair.public_key().into();
		let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944").await.unwrap();
		let nonce = api.tx().account_nonce(&account_id).await.unwrap();
		Self {
			client: Arc::new(api),
			signer: Arc::new(keypair),
			nonce,
		}
	}

	pub async fn submit_transaction(&self, transaction: &[u8]) {
		SubmittableExtrinsic::from_bytes((*self.client).clone(), transaction.to_vec())
			.submit()
			.await
			.unwrap();
	}
}

impl Clone for SubxtClient {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			signer: self.signer.clone(),
			nonce: self.nonce,
		}
	}
}
