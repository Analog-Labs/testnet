
//! Autogenerated weights for `pallet_ocw`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Amars-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: `None`, WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/release/timechain-node
// benchmark
// pallet
// --pallet
// pallet_ocw
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/src/weights/ocw.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_ocw`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ocw::WeightInfo for WeightInfo<T> {
	/// Storage: `Ocw::ShardCollector` (r:1 w:0)
	/// Proof: `Ocw::ShardCollector` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardNetwork` (r:1 w:0)
	/// Proof: `Shards::ShardNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::ShardPublicKey` (r:1 w:1)
	/// Proof: `Shards::ShardPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::UnassignedTasks` (r:1 w:0)
	/// Proof: `Tasks::UnassignedTasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkShards` (r:0 w:1)
	/// Proof: `Tasks::NetworkShards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_tss_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3756`
		// Minimum execution time: 135_000_000 picoseconds.
		Weight::from_parts(138_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3756))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Ocw::ShardCollector` (r:1 w:0)
	/// Proof: `Ocw::ShardCollector` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskCycle` (r:1 w:1)
	/// Proof: `Tasks::TaskCycle` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::Tasks` (r:1 w:0)
	/// Proof: `Tasks::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskShard` (r:1 w:1)
	/// Proof: `Tasks::TaskShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::TaskResults` (r:0 w:1)
	/// Proof: `Tasks::TaskResults` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn submit_task_result() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `3596`
		// Minimum execution time: 121_000_000 picoseconds.
		Weight::from_parts(125_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3596))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}

	fn submit_task_error() -> Weight {
		Weight::default()
	}

	fn submit_task_hash() -> Weight {
		Weight::default()
	}
}
