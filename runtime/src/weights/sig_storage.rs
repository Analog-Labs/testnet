//! Autogenerated weights for `pallet_tesseract_sig_storage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-1-28.us-east-2.compute.internal`, CPU: `Intel(R) Xeon(R) Platinum 8151 CPU @ 3.40GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ../target/release/timechain-node
// benchmark
// pallet
// --pallet
// pallet_tesseract_sig_storage
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../runtime/src/weights/sig_storage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_tesseract_sig_storage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tesseract_sig_storage::WeightInfo for WeightInfo<T> {
	/// Storage: TesseractSigStorage SignatureStoreData (r:0 w:1)
	/// Proof Skipped: TesseractSigStorage SignatureStoreData (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 255]`.
	fn store_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_233_000 picoseconds.
		Weight::from_parts(13_882_200, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 87
			.saturating_add(Weight::from_parts(55, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TesseractSigStorage TssGroupKey (r:0 w:1)
	/// Proof Skipped: TesseractSigStorage TssGroupKey (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[1, 255]`.
	fn submit_tss_group_key(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_532_000 picoseconds.
		Weight::from_parts(13_309_273, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TesseractSigStorage ShardId (r:1 w:1)
	/// Proof Skipped: TesseractSigStorage ShardId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TesseractSigStorage TssShards (r:0 w:1)
	/// Proof Skipped: TesseractSigStorage TssShards (max_values: None, max_size: None, mode: Measured)
	fn register_shard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1491`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_389_000, 0)
			.saturating_add(Weight::from_parts(0, 1491))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// will re run it in correctly configured environment
	fn register_chronicle() -> Weight {
		
		Weight::from_parts(17_389_000, 0)
			.saturating_add(Weight::from_parts(0, 1491))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	// TODO: rerun with prod machine
	fn report_misbehavior() -> Weight {
		T::DbWeight::get().writes(2)
	}

	// TODO: rerun with prod machine
	fn force_set_shard_offline() -> Weight {
		T::DbWeight::get().writes(2)
	}
}
