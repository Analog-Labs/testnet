use crate::{metadata_scope, SubxtClient};
use anyhow::{anyhow, Result};
use time_primitives::{AccountId, NetworkId, ShardId, TssPublicKey};

impl SubxtClient {
	pub async fn networks(&self) -> Result<Vec<NetworkId>> {
		let mut networks = vec![];
		metadata_scope!(self.metadata, {
			let storage = metadata::storage().networks().networks_iter();
			let mut iter = self.client.storage().at_latest().await?.iter(storage).await?;
			while let Some(Ok(kv)) = iter.next().await {
				networks.push(kv.value);
			}
		});
		Ok(networks)
	}

	pub async fn shard_public_key(&self, shard_id: ShardId) -> Result<TssPublicKey> {
		metadata_scope!(self.metadata, {
			let storage = metadata::storage().shards().shard_commitment(shard_id);
			self.client
				.storage()
				.at_latest()
				.await?
				.fetch(&storage)
				.await?
				.ok_or(anyhow!("shard key not found"))
				.map(|v| v[0])
		})
	}

	/* subxt doesn't support decoding keys, use shard_id_counter for now
	pub async fn shards(&self) -> Result<Vec<ShardId>> {
		let mut shards = vec![];
		metadata_scope!(self.metadata, {
			let storage = metadata::storage().shards().shard_state_iter();
			let mut iter = self.client.storage().at_latest().await?.iter(storage).await?;
			while let Some(Ok(kv)) = iter.next().await {
				shards.push(kv.keys);
			}
		});
		Ok(shards)
	}*/

	pub async fn shard_id_counter(&self) -> Result<u64> {
		metadata_scope!(self.metadata, {
			let storage_query = metadata::storage().shards().shard_id_counter();
			Ok(self
				.client
				.storage()
				.at_latest()
				.await?
				.fetch_or_default(&storage_query)
				.await?)
		})
	}

	pub async fn shard_network(&self, shard_id: u64) -> Result<NetworkId> {
		metadata_scope!(self.metadata, {
			let storage_query = metadata::storage().shards().shard_network(shard_id);
			self.client
				.storage()
				.at_latest()
				.await?
				.fetch(&storage_query)
				.await?
				.ok_or(anyhow!("Shard network not found"))
		})
	}

	pub async fn shard_size(&self) -> Result<u16> {
		metadata_scope!(self.metadata, {
			let storage_query = metadata::storage().elections().shard_size();
			self.client
				.storage()
				.at_latest()
				.await?
				.fetch(&storage_query)
				.await?
				.ok_or(anyhow!("Shard size not found"))
		})
	}

	pub async fn shard_threshold(&self) -> Result<u16> {
		metadata_scope!(self.metadata, {
			let storage_query = metadata::storage().elections().shard_threshold();
			self.client
				.storage()
				.at_latest()
				.await?
				.fetch(&storage_query)
				.await?
				.ok_or(anyhow!("Shard size not found"))
		})
	}

	pub async fn member_network(&self, account: &AccountId) -> Result<Option<NetworkId>> {
		metadata_scope!(self.metadata, {
			let account: &subxt::utils::AccountId32 = unsafe { std::mem::transmute(account) };
			let storage_query = metadata::storage().members().member_network(account);
			Ok(self.client.storage().at_latest().await?.fetch(&storage_query).await?)
		})
	}

	pub async fn member_stake(&self, account: &AccountId) -> Result<u128> {
		metadata_scope!(self.metadata, {
			let account: &subxt::utils::AccountId32 = unsafe { std::mem::transmute(account) };
			let storage_query = metadata::storage().members().member_stake(account);
			Ok(self
				.client
				.storage()
				.at_latest()
				.await?
				.fetch(&storage_query)
				.await?
				.unwrap_or_default())
		})
	}

	pub async fn member_online(&self, account: &AccountId) -> Result<bool> {
		metadata_scope!(self.metadata, {
			let account: &subxt::utils::AccountId32 = unsafe { std::mem::transmute(account) };
			let storage_query = metadata::storage().members().member_online(account);
			Ok(self.client.storage().at_latest().await?.fetch(&storage_query).await?.is_some())
		})
	}

	pub async fn member_electable(&self, account: &AccountId) -> Result<bool> {
		metadata_scope!(self.metadata, {
			let account: &subxt::utils::AccountId32 = unsafe { std::mem::transmute(account) };
			let storage_query = metadata::storage().elections().electable(account);
			Ok(self.client.storage().at_latest().await?.fetch(&storage_query).await?.is_some())
		})
	}
}
