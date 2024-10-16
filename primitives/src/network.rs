use crate::Gateway;
use polkadot_sdk::{sp_core::ConstU32, sp_runtime::BoundedVec};
use scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

pub const CHAIN_NAME_LEN: u32 = 50;
pub const CHAIN_NET_LEN: u32 = 50;
pub type NetworkId = u16;

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct ChainNetwork {
	pub chain: BoundedVec<u8, ConstU32<CHAIN_NAME_LEN>>,
	pub net: BoundedVec<u8, ConstU32<CHAIN_NET_LEN>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct Network {
	pub id: NetworkId,
	pub name: ChainNetwork,
	pub gateway: Gateway,
	pub gateway_block: u64,
	pub config: NetworkConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub struct NetworkConfig {
	pub batch_size: u32,
	pub batch_offset: u32,
	pub batch_gas_limit: u128,
	pub shard_task_limit: u32,
}
