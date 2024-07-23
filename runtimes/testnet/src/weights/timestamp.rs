
//! Autogenerated weights for `pallet_timestamp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-17-194.us-east-2.compute.internal`, CPU: `Intel(R) Xeon(R) Platinum 8151 CPU @ 3.40GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ../target/release/timechain-node
// benchmark
// pallet
// --pallet
// pallet_timestamp
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../runtime/src/weights/timestamp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use polkadot_sdk::*;
use polkadot_sdk::*;
use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_timestamp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for WeightInfo<T> {
	/// Storage: Timestamp Now (r:1 w:1)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Babe CurrentSlot (r:1 w:0)
	/// Proof: Babe CurrentSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `138`
		//  Estimated: `1493`
		// Minimum execution time: 15_860_000 picoseconds.
		Weight::from_parts(16_234_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `57`
		//  Estimated: `0`
		// Minimum execution time: 5_624_000 picoseconds.
		Weight::from_parts(5_718_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}
