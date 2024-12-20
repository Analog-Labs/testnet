
//! Autogenerated weights for `pallet_members`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ns1026992`, CPU: `AMD EPYC 4244P 6-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/timechain-node
// benchmark
// pallet
// --pallet
// pallet_members
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./production_weights/members.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_members`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_members::WeightInfo for WeightInfo<T> {
	/// Storage: `Members::MemberStake` (r:1 w:1)
	/// Proof: `Members::MemberStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Members::MemberNetwork` (r:0 w:1)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberRegistered` (r:0 w:1)
	/// Proof: `Members::MemberRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPublicKey` (r:0 w:1)
	/// Proof: `Members::MemberPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberStaker` (r:0 w:1)
	/// Proof: `Members::MemberStaker` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPeerId` (r:0 w:1)
	/// Proof: `Members::MemberPeerId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3593`
		// Minimum execution time: 19_487_000 picoseconds.
		Weight::from_parts(20_217_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Members::MemberStake` (r:1 w:0)
	/// Proof: `Members::MemberStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberNetwork` (r:1 w:0)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberOnline` (r:1 w:1)
	/// Proof: `Members::MemberOnline` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:1 w:0)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Unassigned` (r:1 w:1)
	/// Proof: `Elections::Unassigned` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::TimedOut` (r:1 w:1)
	/// Proof: `Members::TimedOut` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Members::Heartbeat` (r:0 w:1)
	/// Proof: `Members::Heartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn send_heartbeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `500`
		//  Estimated: `3965`
		// Minimum execution time: 18_875_000 picoseconds.
		Weight::from_parts(19_647_000, 0)
			.saturating_add(Weight::from_parts(0, 3965))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Members::MemberStaker` (r:1 w:1)
	/// Proof: `Members::MemberStaker` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberNetwork` (r:1 w:1)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberRegistered` (r:1 w:1)
	/// Proof: `Members::MemberRegistered` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:1 w:0)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberStake` (r:1 w:1)
	/// Proof: `Members::MemberStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Members::MemberPublicKey` (r:0 w:1)
	/// Proof: `Members::MemberPublicKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberPeerId` (r:0 w:1)
	/// Proof: `Members::MemberPeerId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unregister_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `518`
		//  Estimated: `3983`
		// Minimum execution time: 26_590_000 picoseconds.
		Weight::from_parts(27_391_000, 0)
			.saturating_add(Weight::from_parts(0, 3983))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Members::TimedOut` (r:1 w:1)
	/// Proof: `Members::TimedOut` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberNetwork` (r:100 w:0)
	/// Proof: `Members::MemberNetwork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Unassigned` (r:1 w:1)
	/// Proof: `Elections::Unassigned` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Shards::MemberShard` (r:100 w:0)
	/// Proof: `Shards::MemberShard` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::Heartbeat` (r:101 w:100)
	/// Proof: `Members::Heartbeat` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Members::MemberOnline` (r:0 w:100)
	/// Proof: `Members::MemberOnline` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 100]`.
	/// The range of component `b` is `[1, 100]`.
	fn timeout_heartbeats(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + b * (173 ±0)`
		//  Estimated: `3829 + b * (2648 ±0)`
		// Minimum execution time: 15_930_000 picoseconds.
		Weight::from_parts(16_331_000, 0)
			.saturating_add(Weight::from_parts(0, 3829))
			// Standard Error: 4_689
			.saturating_add(Weight::from_parts(9_180_028, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 2648).saturating_mul(b.into()))
	}
}
