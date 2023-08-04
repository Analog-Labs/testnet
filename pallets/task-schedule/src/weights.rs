
//! Autogenerated weights for `task_schedule`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `macs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/timechain-node
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=task_schedule
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=pallets/task-schedule/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `task_schedule`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: PalletProxy ProxyStorage (r:1 w:0)
	/// Proof Skipped: PalletProxy ProxyStorage (max_values: None, max_size: None, mode: Measured)
	/// Storage: TaskSchedule LastKey (r:1 w:1)
	/// Proof Skipped: TaskSchedule LastKey (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TaskSchedule ScheduleStorage (r:0 w:1)
	/// Proof Skipped: TaskSchedule ScheduleStorage (max_values: None, max_size: None, mode: Measured)
	fn create_task() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335`
		//  Estimated: `5955`
		// Minimum execution time: 59_000_000 picoseconds.
		Weight::from_parts(61_000_000, 0)
			.saturating_add(Weight::from_parts(0, 5955))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskSchedule ScheduleStorage (r:1 w:1)
	/// Proof Skipped: TaskSchedule ScheduleStorage (max_values: None, max_size: None, mode: Measured)
	fn submit_result() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `130`
		//  Estimated: `3595`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(53_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3595))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
