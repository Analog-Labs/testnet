
//! Autogenerated weights for `pallet_shards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-11-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-agent-1`, CPU: `AMD EPYC Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./timechain-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_shards
// --extrinsic
// *
// --output
// ./weights/shards.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_shards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_shards::WeightInfo for WeightInfo<T> {
	/// Storage: `Shards::ShardMembers` (r:4 w:1)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardThreshold` (r:1 w:0)
	/// Proof: `Shards::ShardThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPeerId` (r:1 w:0)
	/// Proof: `Members::MemberPeerId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:0 w:1)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:0 w:1)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn commit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757`
		//  Estimated: `11647`
		// Minimum execution time: 515_264_000 picoseconds.
		Weight::from_parts(526_386_000, 0)
			.saturating_add(Weight::from_parts(0, 11647))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Shards::ShardMembers` (r:4 w:1)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkGatewayAddress` (r:1 w:0)
	/// Proof: `Networks::NetworkGatewayAddress` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:0 w:1)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:1)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn ready() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591`
		//  Estimated: `11481`
		// Minimum execution time: 60_332_000 picoseconds.
		Weight::from_parts(64_320_000, 0)
			.saturating_add(Weight::from_parts(0, 11481))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Shards::ShardNetwork` (r:1 w:1)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:1 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:4 w:3)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberRegistered` (r:3 w:0)
	/// Proof: `Members::MemberRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberStaker` (r:3 w:0)
	/// Proof: `Members::MemberStaker` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:0 w:3)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardThreshold` (r:0 w:1)
	/// Proof: `Shards::ShardThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:0 w:1)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberNetwork` (r:0 w:3)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPublicKey` (r:0 w:3)
	/// Proof: `Members::MemberPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPeerId` (r:0 w:3)
	/// Proof: `Members::MemberPeerId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTaskCount` (r:0 w:1)
	/// Proof: `Tasks::ShardTaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:1)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_shard_offline() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459`
		//  Estimated: `11349`
		// Minimum execution time: 110_807_000 picoseconds.
		Weight::from_parts(148_088_000, 0)
			.saturating_add(Weight::from_parts(0, 11349))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(20))
	}
	/// Storage: `Shards::DkgTimeout` (r:6 w:0)
	/// Proof: `Shards::DkgTimeout` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:5 w:5)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:5 w:5)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:5 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:5 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:20 w:15)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberRegistered` (r:3 w:0)
	/// Proof: `Members::MemberRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberStaker` (r:3 w:0)
	/// Proof: `Members::MemberStaker` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:0 w:3)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardThreshold` (r:0 w:5)
	/// Proof: `Shards::ShardThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberNetwork` (r:0 w:3)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPublicKey` (r:0 w:3)
	/// Proof: `Members::MemberPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPeerId` (r:0 w:3)
	/// Proof: `Members::MemberPeerId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTaskCount` (r:0 w:5)
	/// Proof: `Tasks::ShardTaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:5)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 5]`.
	/// The range of component `b` is `[1, 5]`.
	fn timeout_dkgs(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195 + b * (283 ±0)`
		//  Estimated: `8610 + b * (10184 ±0)`
		// Minimum execution time: 117_880_000 picoseconds.
		Weight::from_parts(24_643_241, 0)
			.saturating_add(Weight::from_parts(0, 8610))
			// Standard Error: 198_767
			.saturating_add(Weight::from_parts(106_091_923, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((9_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(12))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 10184).saturating_mul(b.into()))
	}
}
