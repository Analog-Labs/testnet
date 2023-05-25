use super::*;
#[allow(unused)]
use crate::Pallet as TesseractSigStorage;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use time_primitives::{ForeignEventId, TimeId};

pub const ALICE: TimeId = TimeId::new([1u8; 32]);
pub const BOB: TimeId = TimeId::new([2u8; 32]);
pub const CHARLIE: TimeId = TimeId::new([3u8; 32]);

// Check if last event generated by pallet is the one we're expecting
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	store_signature {
		let s in 0..255;
		let signature_data = [s as u8; 64];
		// TODO: extend implementation after same todo fixed in pallet
		let id: ForeignEventId = (s as u128).into();
	}: _(RawOrigin::Signed(ALICE), signature_data, id)
	verify {
		assert!(<SignatureStoreData<T>>::get(id).is_some());
	}

	submit_tss_group_key {
		let s in 1 .. 255;
		let key = [s as u8; 33];
	}: _(RawOrigin::None, s.into(), key)
	verify {
		assert_last_event::<T>(Event::<T>::NewTssGroupKey(s.into(), key).into());
	}

	register_shard {
		let s in 1..256;
	}: _(RawOrigin::Root, s.into(), vec![ALICE, BOB, CHARLIE], Some(ALICE))
	verify {
		assert!(<TssShards<T>>::get(s.into()).is_some());
	}

	impl_benchmark_test_suite!(TesseractSigStorage, crate::mock::new_test_ext(), crate::mock::Test);
}
