use crate::{self as pallet_members};

use polkadot_sdk::{frame_support, frame_system, pallet_balances, sp_core, sp_io, sp_runtime};

use frame_support::derive_impl;
use frame_support::traits::OnInitialize;
use sp_core::{ConstU128, ConstU32, ConstU64};
use sp_runtime::{
	traits::{IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, DispatchError, MultiSignature,
};

use time_primitives::{
	ElectionsInterface, NetworkId, PublicKey, ShardId, ShardsInterface, TssPublicKey,
};

pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;

pub struct MockElections;

impl ElectionsInterface for MockElections {
	type MaxElectionsPerBlock = ConstU32<10>;
	fn member_online(_: &AccountId, _: NetworkId) {}
	fn member_offline(_: &AccountId, _: NetworkId) {}
	fn shard_offline(_network: NetworkId, _members: Vec<AccountId>) {}
}

pub fn pubkey_from_bytes(bytes: [u8; 32]) -> PublicKey {
	PublicKey::Sr25519(sp_core::sr25519::Public::from_raw(bytes))
}

pub struct MockShards;

impl ShardsInterface for MockShards {
	fn member_online(_id: &AccountId, _network: NetworkId) {}
	fn member_offline(_id: &AccountId, _network: NetworkId) {}
	fn is_shard_online(_shard_id: ShardId) -> bool {
		false
	}
	fn is_shard_member(_account: &AccountId) -> bool {
		false
	}
	fn shard_members(_shard_id: ShardId) -> Vec<AccountId> {
		vec![]
	}
	fn shard_network(_shard_id: ShardId) -> Option<NetworkId> {
		None
	}
	fn create_shard(
		_network: NetworkId,
		_members: Vec<AccountId>,
		_threshold: u16,
	) -> Result<ShardId, DispatchError> {
		Ok(0)
	}
	fn next_signer(_shard_id: ShardId) -> PublicKey {
		pubkey_from_bytes([0; 32])
	}
	fn tss_public_key(_shard_id: ShardId) -> Option<TssPublicKey> {
		None
	}
}

frame_support::construct_runtime!(
	pub struct Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Members: pallet_members::{Pallet, Call, Storage, Event<T>},
	}
);

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: <Signature as Verify>::Signer,
		account: AccountId,
		_nonce: u32,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		Some((call, (account, (), ())))
	}
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeTask = RuntimeTask;
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

impl pallet_members::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Elections = MockElections;
	type Shards = MockShards;
	type MinStake = ConstU128<5>;
	type HeartbeatTimeout = ConstU64<10>;
	type MaxTimeoutsPerBlock = ConstU32<100>;
}

/// To from `now` to block `n`.
pub fn roll_to(n: u64) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		Members::on_initialize(i);
	}
}

fn acc_pub(acc_num: u8) -> sp_core::sr25519::Public {
	sp_core::sr25519::Public::from_raw([acc_num; 32])
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(acc_pub(1).into(), 10_000_000_000), (acc_pub(2).into(), 20_000_000_000)],
	}
	.assimilate_storage(&mut storage)
	.unwrap();
	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
