use crate::mock::*;
use crate::{Error, Event, Heartbeat, MemberNetwork, MemberPeerId, MemberStake};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use sp_runtime::{DispatchError, ModuleError};
use time_primitives::{AccountId, MemberStorage, Network, PublicKey};

fn pubkey_from_bytes(bytes: [u8; 32]) -> PublicKey {
	PublicKey::Sr25519(sp_core::sr25519::Public::from_raw(bytes))
}

const A: [u8; 32] = [1u8; 32];
const C: [u8; 32] = [3u8; 32];

#[test]
fn register_member_works() {
	let a: AccountId = A.into();
	new_test_ext().execute_with(|| {
		assert_ok!(Members::register_member(
			RawOrigin::Signed(a.clone()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		System::assert_last_event(Event::<Test>::MemberOnline(a.clone()).into());
		assert_eq!(Members::member_peer_id(&a), Some(A));
		assert_eq!(MemberStake::<Test>::get(&a), 5);
		assert_eq!(MemberPeerId::<Test>::get(&a), Some(A));
		assert_eq!(MemberNetwork::<Test>::get(&a), Some(Network::Ethereum));
		assert!(Heartbeat::<Test>::get(&a).unwrap().is_online);
		assert_eq!(Heartbeat::<Test>::get(&a).unwrap().block, 1);
	});
}

#[test]
fn cannot_register_member_without_balance() {
	let c: AccountId = C.into();
	new_test_ext().execute_with(|| {
		assert_noop!(
			Members::register_member(
				RawOrigin::Signed(c.clone()).into(),
				Network::Ethereum,
				pubkey_from_bytes(C),
				C,
				5,
			),
			DispatchError::Module(ModuleError {
				index: 1,
				error: [2, 0, 0, 0],
				message: Some("InsufficientBalance")
			})
		);
	});
}

#[test]
fn cannot_register_member_below_min_stake_bond() {
	let a: AccountId = A.into();
	new_test_ext().execute_with(|| {
		assert_noop!(
			Members::register_member(
				RawOrigin::Signed(a.clone()).into(),
				Network::Ethereum,
				pubkey_from_bytes(A),
				A,
				4,
			),
			Error::<Test>::BondBelowMinStake
		);
	});
}

#[test]
fn cannot_register_member_twice() {
	new_test_ext().execute_with(|| {
		assert_ok!(Members::register_member(
			RawOrigin::Signed(A.into()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		assert_noop!(
			Members::register_member(
				RawOrigin::Signed(A.into()).into(),
				Network::Ethereum,
				pubkey_from_bytes(A),
				A,
				5,
			),
			Error::<Test>::AlreadyMember
		);
	});
}

#[test]
fn send_heartbeat_works() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_ok!(Members::register_member(
			RawOrigin::Signed(a.clone()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		roll_to(5);
		assert_ok!(Members::send_heartbeat(RawOrigin::Signed(a.clone()).into()));
		System::assert_last_event(Event::<Test>::HeartbeatReceived(a.clone()).into());
		assert!(Heartbeat::<Test>::get(&a).unwrap().is_online);
		assert_eq!(Heartbeat::<Test>::get(&a).unwrap().block, 5);
	});
}

#[test]
fn no_heartbeat_sets_member_offline_after_timeout() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_ok!(Members::register_member(
			RawOrigin::Signed(a.clone()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		roll_to(11);
		assert!(!Heartbeat::<Test>::get(&a).unwrap().is_online);
		assert_eq!(Heartbeat::<Test>::get(&a).unwrap().block, 1);
	});
}

#[test]
fn send_heartbeat_sets_member_back_online_after_timeout() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_ok!(Members::register_member(
			RawOrigin::Signed(a.clone()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		roll_to(11);
		assert!(!Heartbeat::<Test>::get(&a).unwrap().is_online);
		assert_eq!(Heartbeat::<Test>::get(&a).unwrap().block, 1);
		assert_ok!(Members::send_heartbeat(RawOrigin::Signed(a.clone()).into()));
		assert!(Heartbeat::<Test>::get(&a).unwrap().is_online);
		assert_eq!(Heartbeat::<Test>::get(&a).unwrap().block, 11);
	});
}

#[test]
fn cannot_send_heartbeat_if_not_member() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_noop!(
			Members::send_heartbeat(RawOrigin::Signed(a.clone()).into()),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn cannot_unregister_if_not_member() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_noop!(
			Members::unregister_member(RawOrigin::Signed(a.clone()).into()),
			Error::<Test>::NotMember
		);
	});
}

#[test]
fn unregister_member_works() {
	new_test_ext().execute_with(|| {
		let a: AccountId = A.into();
		assert_ok!(Members::register_member(
			RawOrigin::Signed(a.clone()).into(),
			Network::Ethereum,
			pubkey_from_bytes(A),
			A,
			5,
		));
		assert_ok!(Members::unregister_member(RawOrigin::Signed(a.clone()).into()),);
		System::assert_last_event(Event::<Test>::MemberOffline(a.clone()).into());
		assert_eq!(Members::member_peer_id(&a), None);
		assert_eq!(MemberStake::<Test>::get(&a), 0);
		assert_eq!(MemberPeerId::<Test>::get(&a), None);
		assert_eq!(MemberNetwork::<Test>::get(&a), None);
		assert!(Heartbeat::<Test>::get(&a).is_none());
	});
}
