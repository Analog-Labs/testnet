
//! Autogenerated weights for `pallet_tasks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-10-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_tasks
// --extrinsic
// *
// --output
// ./weights/tasks.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_tasks`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tasks::WeightInfo for WeightInfo<T> {
	/// Storage: `Tasks::Tasks` (r:1 w:1)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskOutput` (r:1 w:1)
	/// Proof: `Tasks::TaskOutput` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:1)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkBatchSize` (r:1 w:0)
	/// Proof: `Networks::NetworkBatchSize` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkBatchOffset` (r:1 w:0)
	/// Proof: `Networks::NetworkBatchOffset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskIdCounter` (r:1 w:1)
	/// Proof: `Tasks::TaskIdCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskCount` (r:1 w:1)
	/// Proof: `Tasks::TaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:4 w:0)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:4 w:4)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tasks::ShardTaskCount` (r:1 w:1)
	/// Proof: `Tasks::ShardTaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ExecutedTaskCount` (r:1 w:1)
	/// Proof: `Tasks::ExecutedTaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:0 w:1)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ReadEventsTask` (r:0 w:1)
	/// Proof: `Tasks::ReadEventsTask` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskNetwork` (r:0 w:1)
	/// Proof: `Tasks::TaskNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_task_result() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1380`
		//  Estimated: `12270`
		// Minimum execution time: 624_318_000 picoseconds.
		Weight::from_parts(698_456_000, 0)
			.saturating_add(Weight::from_parts(0, 12270))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(14))
	}
	/// Storage: `Tasks::ReadEventsTask` (r:2 w:0)
	/// Proof: `Tasks::ReadEventsTask` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkShardTaskLimit` (r:1 w:0)
	/// Proof: `Networks::NetworkShardTaskLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:1)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:10001 w:0)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTaskCount` (r:1 w:1)
	/// Proof: `Tasks::ShardTaskCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:10000 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardRegistered` (r:1 w:0)
	/// Proof: `Tasks::ShardRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:0 w:1)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 10000]`.
	/// The range of component `b` is `[1, 10000]`.
	fn schedule_tasks(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4412 + b * (91 ±0)`
		//  Estimated: `10028 + b * (2566 ±0)`
		// Minimum execution time: 59_763_000 picoseconds.
		Weight::from_parts(63_449_000, 0)
			.saturating_add(Weight::from_parts(0, 10028))
			// Standard Error: 10_361
			.saturating_add(Weight::from_parts(11_731_446, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2566).saturating_mul(b.into()))
	}
	/// Storage: `Tasks::ReadEventsTask` (r:2 w:0)
	/// Proof: `Tasks::ReadEventsTask` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkBatchGasLimit` (r:1 w:0)
	/// Proof: `Networks::NetworkBatchGasLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::OpsRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::OpsRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::OpsInsertIndex` (r:1 w:0)
	/// Proof: `Tasks::OpsInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 1000]`.
	/// The range of component `b` is `[1, 1000]`.
	fn prepare_batches(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `958`
		//  Estimated: `6794 + b * (1 ±0)`
		// Minimum execution time: 25_137_000 picoseconds.
		Weight::from_parts(43_753_099, 0)
			.saturating_add(Weight::from_parts(0, 6794))
			// Standard Error: 345
			.saturating_add(Weight::from_parts(14_169, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
	}
}
