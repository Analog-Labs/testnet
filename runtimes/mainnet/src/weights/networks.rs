
//! Autogenerated weights for `pallet_networks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-09-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_networks
// --extrinsic
// *
// --output
// ./weights/networks.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_networks`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_networks::WeightInfo for WeightInfo<T> {
	/// Storage: `Networks::Networks` (r:8 w:1)
	/// Proof: `Networks::Networks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Networks::NetworkIdCounter` (r:1 w:1)
	/// Proof: `Networks::NetworkIdCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `a` is `[1, 1000]`.
	/// The range of component `b` is `[1, 1000]`.
	fn add_network(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `359`
		//  Estimated: `21149`
		// Minimum execution time: 48_802_000 picoseconds.
		Weight::from_parts(53_230_292, 0)
			.saturating_add(Weight::from_parts(0, 21149))
			// Standard Error: 1_120
			.saturating_add(Weight::from_parts(6_231, 0).saturating_mul(a.into()))
			// Standard Error: 1_120
			.saturating_add(Weight::from_parts(476, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Networks::Networks` (r:1 w:1)
	/// Proof: `Networks::Networks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_network() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `332`
		//  Estimated: `3797`
		// Minimum execution time: 19_757_000 picoseconds.
		Weight::from_parts(21_851_000, 0)
			.saturating_add(Weight::from_parts(0, 3797))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
