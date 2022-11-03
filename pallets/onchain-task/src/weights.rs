
//! Autogenerated weights for `pallet_tesseract_sig_storage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/timechain-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_tesseract_sig_storage
// --extrinsic
// store_signature
// --steps
// 50
// --repeat
// 20
// --output
// pallets/tesseract-sig-storage/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]


//use frame_support::{traits::Get, weights::Weight};
use frame_support::{traits::Get, weights::{constants::RocksDbWeight,Weight}};

use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tesseract_sig_storage`.

pub trait WeightInfo {
	fn add_task() -> Weight;
	fn store_onchain_data() -> Weight;
	fn remove_task() -> Weight;
}

pub struct TaskWeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for TaskWeightInfo<T> {
	// Storage: OnChainTask TesseractMembers (r:0 w:1)
    fn add_task() -> Weight {
        Weight::from_ref_time(33_000_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }

    // Storage: OnChainTask TesseractMembers (r:1 w:0)
    // Storage: OnChainTask SignatureStore (r:0 w:1)
    /// The range of component `s` is `[0, 100]`.
    fn store_onchain_data() -> Weight {
        Weight::from_ref_time(38_000_000 as u64)
            // Standard Error: 2_151
            .saturating_add(Weight::from_ref_time(68_175 as u64).saturating_mul(100 as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }

    // Storage: OnChainTask TesseractMembers (r:0 w:1)
    fn remove_task() -> Weight {
        Weight::from_ref_time(33_000_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }


}


impl WeightInfo for () {

		// Storage: OnChainTask TesseractMembers (r:0 w:1)
		fn add_task() -> Weight {
			Weight::from_ref_time(33_000_000 as u64)
				.saturating_add(RocksDbWeight::get().writes(1 as u64))
		}

	fn store_onchain_data() -> Weight {
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	   // Storage: OnChainTask TesseractMembers (r:0 w:1)
	   fn remove_task() -> Weight {
        Weight::from_ref_time(33_000_000 as u64)
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
}