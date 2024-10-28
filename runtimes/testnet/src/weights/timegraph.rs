
//! Autogenerated weights for `pallet_timegraph`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-10-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_timegraph
// --extrinsic
// *
// --output
// ./weights/timegraph.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_timegraph`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timegraph::WeightInfo for WeightInfo<T> {
	/// Storage: `Timegraph::NextDepositSequence` (r:1 w:1)
	/// Proof: `Timegraph::NextDepositSequence` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3468`
		// Minimum execution time: 32_180_000 picoseconds.
		Weight::from_parts(39_453_000, 0)
			.saturating_add(Weight::from_parts(0, 3468))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timegraph::Threshold` (r:1 w:0)
	/// Proof: `Timegraph::Threshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Timegraph::NextWithdrawalSequence` (r:1 w:1)
	/// Proof: `Timegraph::NextWithdrawalSequence` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3468`
		// Minimum execution time: 32_151_000 picoseconds.
		Weight::from_parts(34_825_000, 0)
			.saturating_add(Weight::from_parts(0, 3468))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timegraph::TimegraphAccount` (r:1 w:0)
	/// Proof: `Timegraph::TimegraphAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Timegraph::RewardPoolAccount` (r:1 w:0)
	/// Proof: `Timegraph::RewardPoolAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transfer_to_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `6196`
		// Minimum execution time: 80_481_000 picoseconds.
		Weight::from_parts(120_977_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Timegraph::TimegraphAccount` (r:1 w:0)
	/// Proof: `Timegraph::TimegraphAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Timegraph::RewardPoolAccount` (r:1 w:0)
	/// Proof: `Timegraph::RewardPoolAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_award_to_user() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `3593`
		// Minimum execution time: 78_207_000 picoseconds.
		Weight::from_parts(109_205_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timegraph::TimegraphAccount` (r:1 w:1)
	/// Proof: `Timegraph::TimegraphAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_timegraph_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1488`
		// Minimum execution time: 9_377_000 picoseconds.
		Weight::from_parts(10_480_000, 0)
			.saturating_add(Weight::from_parts(0, 1488))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timegraph::RewardPoolAccount` (r:1 w:1)
	/// Proof: `Timegraph::RewardPoolAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_reward_pool_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1488`
		// Minimum execution time: 9_538_000 picoseconds.
		Weight::from_parts(10_540_000, 0)
			.saturating_add(Weight::from_parts(0, 1488))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timegraph::Threshold` (r:1 w:1)
	/// Proof: `Timegraph::Threshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1488`
		// Minimum execution time: 9_026_000 picoseconds.
		Weight::from_parts(11_061_000, 0)
			.saturating_add(Weight::from_parts(0, 1488))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
