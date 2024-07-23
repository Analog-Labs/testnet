
//! Autogenerated weights for `pallet_timegraph`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-agent-1`, CPU: `AMD EPYC Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("development")`, DB CACHE: 1024

// Executed Command:
// ./timechain-node
// benchmark
// pallet
// --chain
// development
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
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Timegraph::NextDepositSequence` (r:1 w:1)
	/// Proof: `Timegraph::NextDepositSequence` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `192`
		//  Estimated: `3657`
		// Minimum execution time: 66_044_000 picoseconds.
		Weight::from_parts(67_577_000, 0)
			.saturating_add(Weight::from_parts(0, 3657))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Timegraph::NextWithdrawalSequence` (r:1 w:1)
	/// Proof: `Timegraph::NextWithdrawalSequence` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `192`
		//  Estimated: `3657`
		// Minimum execution time: 66_084_000 picoseconds.
		Weight::from_parts(67_665_000, 0)
			.saturating_add(Weight::from_parts(0, 3657))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
