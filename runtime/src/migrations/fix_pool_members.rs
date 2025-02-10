//! Runtime migration to remove buggy pool members

use frame_support::{
	migration::remove_storage_prefix,
	traits::{GetStorageVersion, StorageVersion},
	weights::Weight,
};
use pallet_nomination_pools::PoolMembers;
use sp_runtime::AccountId32;
use sp_std::vec::Vec;

pub struct RemoveBuggyPoolMembersMigration<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config> frame_support::traits::OnRuntimeUpgrade
	for RemoveBuggyPoolMembersMigration<T>
{
	fn on_runtime_upgrade() -> Weight {
		let mut weight: Weight = Weight::zero();

		// Accounts to be removed
		let accounts_to_remove: Vec<AccountId32> = vec![
			AccountId32::from([
				0x42, 0xb1, 0x17, 0xd9, 0x1a, 0x99, 0x2f, 0x1b, 0x02, 0x13, 0x76, 0xd4, 0x7f, 0x63,
				0x78, 0x12, 0x92, 0x99, 0xff, 0x23, 0xb3, 0xa3, 0x67, 0xb8, 0xcd, 0x42, 0x9f, 0x58,
				0x96, 0x88, 0x97, 0xc3,
			]),
			AccountId32::from([
				0x3f, 0x12, 0x98, 0x45, 0xa3, 0xf2, 0x2a, 0x7c, 0x39, 0x87, 0x8b, 0xe9, 0xb5, 0x1f,
				0xc5, 0x94, 0xc4, 0xaa, 0x91, 0xff, 0x23, 0xb2, 0x61, 0xaf, 0x6c, 0x7e, 0x4a, 0xd8,
				0x45, 0xb7, 0xa1, 0x9c,
			]),
			AccountId32::from([
				0x12, 0x76, 0x89, 0x32, 0x74, 0xd4, 0x5b, 0x0a, 0x99, 0x62, 0x88, 0xfa, 0x6b, 0x37,
				0xf2, 0xc5, 0x74, 0x92, 0x23, 0xff, 0x4a, 0xb3, 0x68, 0x91, 0xd2, 0x7e, 0x59, 0xb8,
				0x9a, 0x21, 0x45, 0xdc,
			]),
			AccountId32::from([
				0x4a, 0xa1, 0x6f, 0x32, 0xf5, 0x89, 0x9e, 0x74, 0x23, 0x68, 0xaa, 0xf1, 0x45, 0x7b,
				0x32, 0x92, 0xd8, 0x61, 0xff, 0xc3, 0xb9, 0x23, 0x45, 0x7d, 0x68, 0x91, 0xa2, 0x5e,
				0x4a, 0xb3, 0x67, 0xc2,
			]),
		];

		for account in accounts_to_remove.iter() {
			if PoolMembers::<T>::contains_key(account) {
				PoolMembers::<T>::remove(account);
				weight += T::DbWeight::get().writes(1);
			}
		}

		weight
	}
}
