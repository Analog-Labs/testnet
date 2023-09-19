use crate::{mock::*, Error, Event, ShardSize, ShardThreshold, Unassigned};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use time_primitives::{ElectionsInterface, MemberEvents, Network};

#[test]
fn new_member_online_inserts_unassigned() {
	let a: AccountId = [1u8; 32].into();
	new_test_ext().execute_with(|| {
		Elections::member_online(&a, Network::Ethereum);
		assert!(Unassigned::<Test>::get(Network::Ethereum, &a).is_some());
	});
}

#[test]
fn shard_size_new_members_online_creates_shard() {
	let a: AccountId = [1u8; 32].into();
	let b: AccountId = [2u8; 32].into();
	let c: AccountId = [3u8; 32].into();
	new_test_ext().execute_with(|| {
		Elections::member_online(&a, Network::Ethereum);
		assert!(Unassigned::<Test>::get(Network::Ethereum, &a).is_some());
		Elections::member_online(&b, Network::Ethereum);
		assert!(Unassigned::<Test>::get(Network::Ethereum, &b).is_some());
		Elections::member_online(&c, Network::Ethereum);
		System::assert_last_event(
			pallet_shards::Event::<Test>::ShardCreated(0, Network::Ethereum).into(),
		);
		for member in [a, b, c] {
			assert!(Unassigned::<Test>::get(Network::Ethereum, member).is_none());
		}
		assert_eq!(pallet_shards::ShardThreshold::<Test>::get(0), Some(2));
	});
}

#[test]
fn member_offline_removes_unassigned() {
	let a: AccountId = [1u8; 32].into();
	new_test_ext().execute_with(|| {
		Elections::member_online(&a, Network::Ethereum);
		assert!(Unassigned::<Test>::get(Network::Ethereum, &a).is_some());
		Elections::member_offline(&a, Network::Ethereum);
		assert!(Unassigned::<Test>::get(Network::Ethereum, &a).is_none());
	});
}

#[test]
fn shard_offline_automatically_creates_new_shard() {
	let a: AccountId = [1u8; 32].into();
	let b: AccountId = [2u8; 32].into();
	let c: AccountId = [3u8; 32].into();
	new_test_ext().execute_with(|| {
		Elections::member_online(&a, Network::Ethereum);
		Elections::member_online(&b, Network::Ethereum);
		Elections::member_online(&c, Network::Ethereum);
		System::assert_last_event(
			pallet_shards::Event::<Test>::ShardCreated(0, Network::Ethereum).into(),
		);
		Elections::shard_offline(Network::Ethereum, [a, b, c].to_vec());
		System::assert_last_event(
			pallet_shards::Event::<Test>::ShardCreated(1, Network::Ethereum).into(),
		);
	});
}

#[test]
fn set_shard_config_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(ShardSize::<Test>::get(), 3);
		assert_eq!(ShardThreshold::<Test>::get(), 2);
		assert_ok!(Elections::set_shard_config(RawOrigin::Root.into(), 2, 1));
		System::assert_last_event(Event::<Test>::ShardConfigSet(2, 1).into());
		assert_eq!(ShardSize::<Test>::get(), 2);
		assert_eq!(ShardThreshold::<Test>::get(), 1);
		assert_noop!(
			Elections::set_shard_config(RawOrigin::Root.into(), 1, 2),
			Error::<Test>::SizeGEQThreshold
		);
	});
}
