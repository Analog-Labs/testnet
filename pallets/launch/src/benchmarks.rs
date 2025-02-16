#![cfg(feature = "runtime-benchmarks")]
use crate::{allocation::Allocation, BalanceOf, Call, Config, CurrencyOf, Pallet};

//use super::mock_helpers::*;
use polkadot_sdk::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use time_primitives::{Balance, TARGET_ISSUANCE};

#[benchmarks(
	where
		BalanceOf<T>: From<Balance>,
)]
mod benchmarks {
	use super::*;

	//use polkadot_sdk::pallet_balances;
	use polkadot_sdk::frame_support::traits::Currency;

	#[benchmark]
	fn set_bridged_issuance() {
		let bridge_account = Allocation::Bridged.account_id::<T>();
		let bridge_issuance = BalanceOf::<T>::from(TARGET_ISSUANCE);
		let _ = CurrencyOf::<T>::deposit_creating(&bridge_account, bridge_issuance);

		#[extrinsic_call]
		_(RawOrigin::Root, bridge_issuance);

		//assert_eq!(pallet_balances::Pallet::<T>::balance_locked(*b"bridged0", &bridge_account),bridge_issuance);
	}
}
