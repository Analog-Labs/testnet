
//! Autogenerated weights for `pallet_shards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-agent-1`, CPU: `AMD EPYC Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("staging")`, DB CACHE: 1024

// Executed Command:
// ./timechain-node
// benchmark
// pallet
// --chain
// staging
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
		//  Measured:  `828`
		//  Estimated: `11718`
		// Minimum execution time: 518_201_000 picoseconds.
		Weight::from_parts(522_729_000, 0)
			.saturating_add(Weight::from_parts(0, 11718))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Shards::ShardMembers` (r:4 w:1)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Gateway` (r:1 w:0)
	/// Proof: `Tasks::Gateway` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:1 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardRegistered` (r:1 w:0)
	/// Proof: `Tasks::ShardRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTaskLimit` (r:1 w:0)
	/// Proof: `Tasks::ShardTaskLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UASystemTasksInsertIndex` (r:1 w:0)
	/// Proof: `Tasks::UASystemTasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UASystemTasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UASystemTasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksInsertIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:0 w:1)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:1)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn ready() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `11594`
		// Minimum execution time: 97_843_000 picoseconds.
		Weight::from_parts(100_577_000, 0)
			.saturating_add(Weight::from_parts(0, 11594))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Shards::ShardNetwork` (r:1 w:1)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:2 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardRegistered` (r:1 w:0)
	/// Proof: `Tasks::ShardRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedSystemTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedSystemTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:4 w:3)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::ShardSize` (r:1 w:0)
	/// Proof: `Elections::ShardSize` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Unassigned` (r:4 w:3)
	/// Proof: `Elections::Unassigned` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberOnline` (r:3 w:0)
	/// Proof: `Members::MemberOnline` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:0 w:3)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardThreshold` (r:0 w:1)
	/// Proof: `Shards::ShardThreshold` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:0 w:1)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:1)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_shard_offline() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `712`
		//  Estimated: `11602`
		// Minimum execution time: 115_317_000 picoseconds.
		Weight::from_parts(116_869_000, 0)
			.saturating_add(Weight::from_parts(0, 11602))
			.saturating_add(T::DbWeight::get().reads(18))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	/// Storage: `Shards::MemberShard` (r:1 w:0)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn member_offline() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3541`
		// Minimum execution time: 4_248_000 picoseconds.
		Weight::from_parts(4_518_000, 0)
			.saturating_add(Weight::from_parts(0, 3541))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
