use crate::{mock::*, Error};

use polkadot_sdk::{
	frame_support::{self, traits::ExistenceRequirement},
	frame_system, sp_runtime,
};

use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use sp_runtime::{traits::Get, DispatchError, DispatchResult, ModuleError};

use time_primitives::{AccountId, NetworkId, Task, TasksInterface};

const ETHEREUM: NetworkId = 0;

fn register_gateway(network: NetworkId, block: u64) {
	Tasks::gateway_registered(network, block);
}

#[test]
fn test_teleport_creates_gmp_message() {
	new_test_ext().execute_with(|| {
		assert!(Tasks::get_task(0).is_none());
		register_gateway(ETHEREUM, 42);
		assert_eq!(Tasks::get_task(1), Some(Task::ReadGatewayEvents { blocks: 42..47 }));

		assert!(Tasks::get_task(2).is_none());

		assert_ok!(Bridge::do_teleport(
			acc_pub(0).into(),
			ETHEREUM,
			acc_pub(1).into(),
			123_000,
			ExistenceRequirement::KeepAlive
		));

		assert_eq!(Tasks::get_task(2), Some(Task::SubmitGatewayMessage { batch_id: 1 }));
	})
}

#[test]
fn cannot_teleport_to_inactive_network() {
	new_test_ext().execute_with(|| {
		assert!(Tasks::get_task(0).is_none());
		register_gateway(ETHEREUM, 42);
		assert_eq!(Tasks::get_task(1), Some(Task::ReadGatewayEvents { blocks: 42..47 }));

		assert_noop!(
			Bridge::do_teleport(
				acc_pub(0).into(),
				ETHEREUM,
				acc_pub(1).into(),
				123_000,
				ExistenceRequirement::KeepAlive
			),
			Error::<Test>::NetworkDisabled,
		);
	})
}

#[test]
fn cannot_register_timechain_network() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Bridge::do_register_network(
				<Test as pallet_networks::Config>::TimechainNetworkId::get(),
				Default::default(),
				Default::default(),
			),
			Error::<Test>::NetworkAlreadyExists,
		);
	})
}
