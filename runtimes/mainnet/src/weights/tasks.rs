
//! Autogenerated weights for `pallet_tasks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
	/// Storage: `Shards::ShardNetwork` (r:2 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardState` (r:1 w:0)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:4 w:0)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskIdCounter` (r:1 w:1)
	/// Proof: `Tasks::TaskIdCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkReadReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkReadReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkWriteReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkWriteReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkSendMessageReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkSendMessageReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tasks::UATasksInsertIndex` (r:1 w:1)
	/// Proof: `Tasks::UATasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskRewardConfig` (r:0 w:1)
	/// Proof: `Tasks::TaskRewardConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:0 w:1)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:0 w:1)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:0 w:1)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATaskIndex` (r:0 w:1)
	/// Proof: `Tasks::UATaskIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 10000]`.
	fn create_task(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `11556`
		// Minimum execution time: 133_481_000 picoseconds.
		Weight::from_parts(140_074_931, 0)
			.saturating_add(Weight::from_parts(0, 11556))
			// Standard Error: 140
			.saturating_add(Weight::from_parts(1_733, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskOutput` (r:1 w:1)
	/// Proof: `Tasks::TaskOutput` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:1 w:0)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:1)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::PhaseStart` (r:1 w:1)
	/// Proof: `Tasks::PhaseStart` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskRewardConfig` (r:1 w:1)
	/// Proof: `Tasks::TaskRewardConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:4 w:0)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:4 w:4)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tasks::SignerPayout` (r:1 w:0)
	/// Proof: `Tasks::SignerPayout` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:1 w:1)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_result() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1710`
		//  Estimated: `12600`
		// Minimum execution time: 632_104_000 picoseconds.
		Weight::from_parts(669_624_000, 0)
			.saturating_add(Weight::from_parts(0, 12600))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:1 w:1)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSigner` (r:1 w:0)
	/// Proof: `Tasks::TaskSigner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:0)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskRewardConfig` (r:1 w:0)
	/// Proof: `Tasks::TaskRewardConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::PhaseStart` (r:1 w:1)
	/// Proof: `Tasks::PhaseStart` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskHash` (r:0 w:1)
	/// Proof: `Tasks::TaskHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::SignerPayout` (r:0 w:1)
	/// Proof: `Tasks::SignerPayout` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `628`
		//  Estimated: `4093`
		// Minimum execution time: 48_361_000 picoseconds.
		Weight::from_parts(51_787_000, 0)
			.saturating_add(Weight::from_parts(0, 4093))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Tasks::TaskSignature` (r:1 w:1)
	/// Proof: `Tasks::TaskSignature` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:1 w:1)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:0)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardCommitment` (r:1 w:0)
	/// Proof: `Shards::ShardCommitment` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Gateway` (r:1 w:0)
	/// Proof: `Tasks::Gateway` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardMembers` (r:7 w:0)
	/// Proof: `Shards::ShardMembers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::SignerIndex` (r:1 w:1)
	/// Proof: `Shards::SignerIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPublicKey` (r:1 w:0)
	/// Proof: `Members::MemberPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::PhaseStart` (r:0 w:1)
	/// Proof: `Tasks::PhaseStart` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSigner` (r:0 w:1)
	/// Proof: `Tasks::TaskSigner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2063`
		//  Estimated: `20378`
		// Minimum execution time: 500_919_000 picoseconds.
		Weight::from_parts(521_296_000, 0)
			.saturating_add(Weight::from_parts(0, 20378))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Shards::ShardState` (r:1 w:0)
	/// Proof: `Shards::ShardState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:2 w:0)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardRegistered` (r:1 w:1)
	/// Proof: `Tasks::ShardRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedSystemTasks` (r:1 w:1)
	/// Proof: `Tasks::UnassignedSystemTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:1 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Gateway` (r:1 w:1)
	/// Proof: `Tasks::Gateway` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkBatchSize` (r:1 w:0)
	/// Proof: `Tasks::NetworkBatchSize` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkOffset` (r:1 w:0)
	/// Proof: `Tasks::NetworkOffset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::ShardSize` (r:1 w:0)
	/// Proof: `Elections::ShardSize` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskIdCounter` (r:1 w:1)
	/// Proof: `Tasks::TaskIdCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkReadReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkReadReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkWriteReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkWriteReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkSendMessageReward` (r:1 w:0)
	/// Proof: `Tasks::NetworkSendMessageReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tasks::UASystemTasksInsertIndex` (r:1 w:1)
	/// Proof: `Tasks::UASystemTasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UASystemTasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UASystemTasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskRewardConfig` (r:0 w:1)
	/// Proof: `Tasks::TaskRewardConfig` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::RecvTasks` (r:0 w:1)
	/// Proof: `Tasks::RecvTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:0 w:1)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:0 w:1)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATaskIndex` (r:0 w:1)
	/// Proof: `Tasks::UATaskIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_gateway() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `716`
		//  Estimated: `6656`
		// Minimum execution time: 132_528_000 picoseconds.
		Weight::from_parts(137_187_000, 0)
			.saturating_add(Weight::from_parts(0, 6656))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	/// Storage: `Tasks::NetworkReadReward` (r:0 w:1)
	/// Proof: `Tasks::NetworkReadReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_read_task_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_387_000 picoseconds.
		Weight::from_parts(9_778_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Tasks::NetworkWriteReward` (r:0 w:1)
	/// Proof: `Tasks::NetworkWriteReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_write_task_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_387_000 picoseconds.
		Weight::from_parts(11_501_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Tasks::NetworkSendMessageReward` (r:0 w:1)
	/// Proof: `Tasks::NetworkSendMessageReward` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_send_message_task_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_517_000 picoseconds.
		Weight::from_parts(10_459_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:1)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATaskIndex` (r:1 w:1)
	/// Proof: `Tasks::UATaskIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksInsertIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskHash` (r:0 w:1)
	/// Proof: `Tasks::TaskHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:0 w:1)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSigner` (r:0 w:1)
	/// Proof: `Tasks::TaskSigner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:0 w:1)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSignature` (r:0 w:1)
	/// Proof: `Tasks::TaskSignature` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskOutput` (r:0 w:1)
	/// Proof: `Tasks::TaskOutput` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_cancel_task() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `564`
		//  Estimated: `4029`
		// Minimum execution time: 54_151_000 picoseconds.
		Weight::from_parts(57_428_000, 0)
			.saturating_add(Weight::from_parts(0, 4029))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Tasks::UnassignedTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedSystemTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedSystemTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:11 w:10)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:10 w:10)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATaskIndex` (r:10 w:10)
	/// Proof: `Tasks::UATaskIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:10 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksInsertIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskHash` (r:0 w:10)
	/// Proof: `Tasks::TaskHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSigner` (r:0 w:10)
	/// Proof: `Tasks::TaskSigner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:0 w:10)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskSignature` (r:0 w:10)
	/// Proof: `Tasks::TaskSignature` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskOutput` (r:0 w:10)
	/// Proof: `Tasks::TaskOutput` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 10]`.
	fn sudo_cancel_tasks(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `724 + b * (247 ±0)`
		//  Estimated: `4197 + b * (2718 ±0)`
		// Minimum execution time: 75_231_000 picoseconds.
		Weight::from_parts(33_424_990, 0)
			.saturating_add(Weight::from_parts(0, 4197))
			// Standard Error: 150_582
			.saturating_add(Weight::from_parts(43_408_598, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 2718).saturating_mul(b.into()))
	}
	/// Storage: `Tasks::TaskShard` (r:11 w:10)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:10 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksInsertIndex` (r:1 w:1)
	/// Proof: `Tasks::UATasksInsertIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATasksRemoveIndex` (r:1 w:0)
	/// Proof: `Tasks::UATasksRemoveIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:11 w:10)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:10 w:10)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedSystemTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedSystemTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskPhaseState` (r:0 w:10)
	/// Proof: `Tasks::TaskPhaseState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UATaskIndex` (r:0 w:10)
	/// Proof: `Tasks::UATaskIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 10]`.
	fn reset_tasks(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497 + b * (212 ±0)`
		//  Estimated: `3972 + b * (2683 ±0)`
		// Minimum execution time: 62_758_000 picoseconds.
		Weight::from_parts(31_822_102, 0)
			.saturating_add(Weight::from_parts(0, 3972))
			// Standard Error: 98_327
			.saturating_add(Weight::from_parts(35_544_355, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 2683).saturating_mul(b.into()))
	}
	/// Storage: `Tasks::ShardTaskLimit` (r:0 w:1)
	/// Proof: `Tasks::ShardTaskLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_shard_task_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_688_000 picoseconds.
		Weight::from_parts(10_780_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Tasks::Gateway` (r:1 w:1)
	/// Proof: `Tasks::Gateway` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardRegistered` (r:1 w:1)
	/// Proof: `Tasks::ShardRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedSystemTasks` (r:65 w:0)
	/// Proof: `Tasks::UnassignedSystemTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:64 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:0)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::ShardTasks` (r:1 w:0)
	/// Proof: `Tasks::ShardTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskOutput` (r:0 w:1)
	/// Proof: `Tasks::TaskOutput` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 10]`.
	fn unregister_gateways(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + b * (569 ±0)`
		//  Estimated: `6444 + b * (12303 ±124)`
		// Minimum execution time: 44_864_000 picoseconds.
		Weight::from_parts(46_978_000, 0)
			.saturating_add(Weight::from_parts(0, 6444))
			// Standard Error: 399_537
			.saturating_add(Weight::from_parts(41_558_120, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((10_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 12303).saturating_mul(b.into()))
	}
	/// Storage: `Tasks::NetworkBatchSize` (r:0 w:1)
	/// Proof: `Tasks::NetworkBatchSize` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkOffset` (r:0 w:1)
	/// Proof: `Tasks::NetworkOffset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_batch_size() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_642_000 picoseconds.
		Weight::from_parts(12_514_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
