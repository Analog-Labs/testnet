
//! Autogenerated weights for `pallet_networks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
	/// The range of component `a` is `[1, 1000]`.
	/// The range of component `b` is `[1, 1000]`.
	fn add_network(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `359`
		//  Estimated: `21149`
		// Minimum execution time: 48_322_000 picoseconds.
		Weight::from_parts(49_145_320, 0)
			.saturating_add(Weight::from_parts(0, 21149))
			// Standard Error: 127
			.saturating_add(Weight::from_parts(1_462, 0).saturating_mul(a.into()))
			// Standard Error: 127
			.saturating_add(Weight::from_parts(1_284, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: `Tasks::NetworkBatchSize` (r:0 w:1)
	/// Proof: `Tasks::NetworkBatchSize` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkOffset` (r:0 w:1)
	/// Proof: `Tasks::NetworkOffset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_gateway() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_392_000 picoseconds.
		Weight::from_parts(11_992_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: `Tasks::NetworkBatchSize` (r:0 w:1)
	/// Proof: `Tasks::NetworkBatchSize` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tasks::NetworkOffset` (r:0 w:1)
	/// Proof: `Tasks::NetworkOffset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_batch_size() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_392_000 picoseconds.
		Weight::from_parts(11_992_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	fn set_batch_gas_limit() -> Weight {
		Weight::default()
	}
}
