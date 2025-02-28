
//! Autogenerated weights for `pallet_launch`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2025-02-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ns1026992`, CPU: `AMD EPYC 4244P 6-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./node-bench-mainnet-e1172636
// benchmark
// pallet
// --pallet
// pallet_launch
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/src/weights/mainnet/pallet_launch.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_launch`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_launch::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn lock_operational() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `4764`
		// Minimum execution time: 15_379_000 picoseconds.
		Weight::from_parts(16_080_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
