//! Runtime migration to remove buggy pool members
use frame_support::{traits::Get, weights::Weight};
use pallet_nomination_pools::PoolMembers;
use polkadot_sdk::*;
use scale_info::prelude::vec;
use sp_runtime::AccountId32;
use sp_std::vec::Vec;

pub struct RemoveBuggyPoolMembersMigration<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config + pallet_nomination_pools::Config>
	frame_support::traits::OnRuntimeUpgrade for RemoveBuggyPoolMembersMigration<T>
where
	T::AccountId: From<AccountId32>,
{
	fn on_runtime_upgrade() -> Weight {
		let mut weight: Weight = Weight::zero();

		// Accounts to be removed
		let accounts_to_remove: Vec<T::AccountId> = vec![
			AccountId32::new([
				0xB7, 0x98, 0x2C, 0xE8, 0x0E, 0x4B, 0x52, 0xF2, 0xE8, 0x0A, 0xAF, 0x36, 0xD1, 0x8B,
				0x1E, 0xBA, 0x1A, 0x32, 0x00, 0x5F, 0xFB, 0xEF, 0xD9, 0x52, 0x96, 0x22, 0x27, 0xF2,
				0xF4, 0xDB, 0x30, 0x9,
			])
			.into(),
			AccountId32::new([
				0xAE, 0xED, 0xB1, 0x73, 0x8F, 0xB7, 0x13, 0x3D, 0xA2, 0x40, 0x21, 0x3A, 0xB0, 0xF0,
				0x91, 0x6E, 0xB7, 0xAB, 0xAF, 0x41, 0x40, 0x69, 0x3B, 0x7A, 0x3F, 0x54, 0x31, 0x17,
				0x32, 0xBD, 0xDA, 0x33,
			])
			.into(),
			AccountId32::new([
				0xB2, 0x07, 0x54, 0xDC, 0xEA, 0xBB, 0x4F, 0x4E, 0x9F, 0xA4, 0xB8, 0x92, 0x37, 0x11,
				0xA8, 0x7D, 0xDB, 0x26, 0x33, 0x68, 0x75, 0x0D, 0xB3, 0xA8, 0xF0, 0x5E, 0x4E, 0x5E,
				0x5E, 0xA5, 0xA1, 0x55,
			])
			.into(),
			AccountId32::new([
				0xAA, 0x87, 0x03, 0x19, 0x4B, 0x1B, 0x55, 0x5A, 0x1D, 0xC1, 0xBA, 0x17, 0xB0, 0x99,
				0x46, 0x7D, 0xB4, 0x57, 0x97, 0x44, 0x4E, 0x9C, 0xC4, 0xAE, 0xCF, 0x2A, 0x5F, 0x95,
				0xD2, 0x75, 0xB5, 0x7A,
			])
			.into(),
		];

		for account in accounts_to_remove.iter() {
			if PoolMembers::<T>::contains_key(account) {
				PoolMembers::<T>::remove(account);
				log::info!("Removed pool member: {account:?}");
				weight += <T as frame_system::Config>::DbWeight::get().writes(1);
			}
		}

		weight
	}
}

#[test]
fn test_pool_member_account_removal() {
	use sp_core::crypto::Ss58Codec;
	use sp_runtime::AccountId32;

	// Provided SS58 addresses
	let provided_accounts = [
		"an67vKKCtXjWYUaErzKp5E97dDnyt39HNZMwTAReK1PLUVdbA",
		"an9XX2NvQXRvFBwMdNiaaXbAKHN4tA6oi7xGVJbffR84AvcDw",
		"an9banSywn1r4nUkpVvFaBU5Jn5T32W9YHVvG33qo7AsrC1ci",
		"an9RkL7gJoDerqU71knygQNzPC3Gb1gx5cTJha7h1xinLDWwc",
	];

	// Convert SS58 to AccountId32 (raw bytes)
	let decoded_accounts: Vec<AccountId32> = provided_accounts
		.iter()
		.map(|acc| AccountId32::from_string(acc).expect("Invalid SS58 Address"))
		.collect();

	// Manually specified accounts in the migration
	let migration_accounts = [
		AccountId32::new([
			0xB7, 0x98, 0x2C, 0xE8, 0x0E, 0x4B, 0x52, 0xF2, 0xE8, 0x0A, 0xAF, 0x36, 0xD1, 0x8B,
			0x1E, 0xBA, 0x1A, 0x32, 0x00, 0x5F, 0xFB, 0xEF, 0xD9, 0x52, 0x96, 0x22, 0x27, 0xF2,
			0xF4, 0xDB, 0x30, 0x9,
		]),
		AccountId32::new([
			0xAE, 0xED, 0xB1, 0x73, 0x8F, 0xB7, 0x13, 0x3D, 0xA2, 0x40, 0x21, 0x3A, 0xB0, 0xF0,
			0x91, 0x6E, 0xB7, 0xAB, 0xAF, 0x41, 0x40, 0x69, 0x3B, 0x7A, 0x3F, 0x54, 0x31, 0x17,
			0x32, 0xBD, 0xDA, 0x33,
		]),
		AccountId32::new([
			0xB2, 0x07, 0x54, 0xDC, 0xEA, 0xBB, 0x4F, 0x4E, 0x9F, 0xA4, 0xB8, 0x92, 0x37, 0x11,
			0xA8, 0x7D, 0xDB, 0x26, 0x33, 0x68, 0x75, 0x0D, 0xB3, 0xA8, 0xF0, 0x5E, 0x4E, 0x5E,
			0x5E, 0xA5, 0xA1, 0x55,
		]),
		AccountId32::new([
			0xAA, 0x87, 0x03, 0x19, 0x4B, 0x1B, 0x55, 0x5A, 0x1D, 0xC1, 0xBA, 0x17, 0xB0, 0x99,
			0x46, 0x7D, 0xB4, 0x57, 0x97, 0x44, 0x4E, 0x9C, 0xC4, 0xAE, 0xCF, 0x2A, 0x5F, 0x95,
			0xD2, 0x75, 0xB5, 0x7A,
		]),
	];

	for (i, (provided, migration)) in decoded_accounts.iter().zip(&migration_accounts).enumerate() {
		if provided != migration {
			println!(
				"Mismatch at index {}: Provided ({:?}) != Migration ({:?})",
				i, provided, migration
			);
		}
	}

	// Assert equality with a clear error message
	assert_eq!(
		decoded_accounts, migration_accounts,
		"Account byte mismatch! Please update the migration bytes."
	);
}
