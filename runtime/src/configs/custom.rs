use polkadot_sdk::*;

use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32},
};

// Can't use `FungibleAdapter` here until Treasury pallet migrates to fungibles
// <https://github.com/paritytech/polkadot-sdk/issues/226>
#[allow(deprecated)]
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};

use time_primitives::ANLOG;
// Local module imports
use crate::{
	weights, AccountId, Balance, Balances, Elections, Members, Networks, Runtime, RuntimeEvent,
	Shards, Tasks,
};

#[cfg(not(feature = "testnet"))]
use super::governance::EnsureRootOrHalfTechnical;
#[cfg(feature = "testnet")]
use super::governance::EnsureRootOrTechnicalMember;

// Custom pallet config
parameter_types! {
	pub IndexerReward: Balance = ANLOG;
}

#[cfg(not(feature = "testnet"))]
/// Default admin origin for all chronicle related pallets
type ChronicleAdmin = EnsureRootOrHalfTechnical;

#[cfg(feature = "testnet")]
/// Development admin origin for all chronicle related pallets
type ChronicleAdmin = EnsureRootOrTechnicalMember;

impl pallet_members::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_members::WeightInfo<Runtime>;
	type Elections = Elections;
	type Shards = Shards;
	type MinStake = ConstU128<1>;
	type HeartbeatTimeout = ConstU32<300>;
	type MaxTimeoutsPerBlock = ConstU32<25>;
}

impl pallet_elections::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = ChronicleAdmin;
	type WeightInfo = weights::pallet_elections::WeightInfo<Runtime>;
	type Members = Members;
	type Shards = Shards;
	type Networks = Networks;
	type MaxElectionsPerBlock = ConstU32<25>;
}

impl pallet_shards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = ChronicleAdmin;
	type WeightInfo = weights::pallet_shards::WeightInfo<Runtime>;
	type Members = Members;
	type Elections = Elections;
	type Tasks = Tasks;
	type DkgTimeout = ConstU32<10>;
}

impl pallet_tasks::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = ChronicleAdmin;
	type WeightInfo = weights::pallet_tasks::WeightInfo<Runtime>;
	type Networks = Networks;
	type Shards = Shards;
	type MaxTasksPerBlock = ConstU32<50>;
	type MaxBatchesPerBlock = ConstU32<10>;
}

parameter_types! {
 pub const InitialRewardPoolAccount: AccountId = AccountId::new([0_u8; 32]);
 pub const InitialTimegraphAccount: AccountId = AccountId::new([0_u8; 32]);
 pub const InitialThreshold: Balance = 1000;
}

impl pallet_timegraph::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_timegraph::WeightInfo<Runtime>;
	type Currency = Balances;
	type InitialRewardPoolAccount = InitialRewardPoolAccount;
	type InitialTimegraphAccount = InitialTimegraphAccount;
	type InitialThreshold = InitialThreshold;
	type AdminOrigin = EnsureRootOrTechnicalMember;
}

impl pallet_networks::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = ChronicleAdmin;
	type WeightInfo = weights::pallet_networks::WeightInfo<Runtime>;
	type Tasks = Tasks;
}

impl pallet_dmail::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_dmail::WeightInfo<Runtime>;
}

#[cfg(test)]
mod test {
	use polkadot_sdk::*;

	use frame_support::pallet_prelude::Get;
	use frame_support::weights::Weight;

	use crate::{Runtime, AVERAGE_ON_INITIALIZE_RATIO, MAXIMUM_BLOCK_WEIGHT};
	use time_primitives::ON_INITIALIZE_BOUNDS; // AVERAGE_ON_INITIALIZE_RATIO};

	use pallet_elections::WeightInfo as ElectionsWeights;
	use pallet_members::WeightInfo as MembersWeights;
	use pallet_tasks::WeightInfo as TasksWeights;

	#[test]
	fn max_batches_per_block() {
		let avg_on_initialize: Weight =
			ON_INITIALIZE_BOUNDS.batches * (AVERAGE_ON_INITIALIZE_RATIO * MAXIMUM_BLOCK_WEIGHT);
		assert!(
			<Runtime as pallet_tasks::Config>::WeightInfo::prepare_batches(1)
				.all_lte(avg_on_initialize),
			"BUG: Starting a single batch consumes more weight than available in on-initialize"
		);
		assert!(
			<Runtime as pallet_tasks::Config>::WeightInfo::prepare_batches(1)
				.all_lte(<Runtime as pallet_tasks::Config>::WeightInfo::prepare_batches(2)),
			"BUG: Starting 1 batch consumes more weight than starting 2"
		);
		let mut num_batches: u32 = 2;
		while <Runtime as pallet_tasks::Config>::WeightInfo::prepare_batches(num_batches)
			.all_lt(avg_on_initialize)
		{
			num_batches += 1;
			if num_batches == 10_000_000 {
				// 10_000_000 batches started; halting to break out of loop
				break;
			}
		}
		let max_batches_per_block_configured: u32 =
			<Runtime as pallet_tasks::Config>::MaxBatchesPerBlock::get();
		assert!(
			max_batches_per_block_configured <= num_batches,
			"MaxBatchesPerBlock {max_batches_per_block_configured} > max number of batches per block tested = {num_batches}"
		);
	}

	#[test]
	fn max_heartbeat_timeouts_per_block() {
		let avg_on_initialize: Weight =
			ON_INITIALIZE_BOUNDS.heartbeats * (AVERAGE_ON_INITIALIZE_RATIO * MAXIMUM_BLOCK_WEIGHT);
		assert!(
			<Runtime as pallet_members::Config>::WeightInfo::timeout_heartbeats(1)
				.all_lte(avg_on_initialize),
			"BUG: One Heartbeat timeout consumes more weight than available in on-initialize"
		);
		assert!(
			<Runtime as pallet_members::Config>::WeightInfo::timeout_heartbeats(1)
				.all_lte(<Runtime as pallet_members::Config>::WeightInfo::timeout_heartbeats(2)),
			"BUG: 1 Heartbeat timeout consumes more weight than 2 Heartbeat timeouts"
		);
		let mut num_timeouts: u32 = 2;
		while <Runtime as pallet_members::Config>::WeightInfo::timeout_heartbeats(num_timeouts)
			.all_lt(avg_on_initialize)
		{
			num_timeouts += 1;
			if num_timeouts == 10_000_000 {
				// 10_000_000 timeouts; halting to break out of loop
				break;
			}
		}
		let max_timeouts_per_block: u32 =
			<Runtime as pallet_members::Config>::MaxTimeoutsPerBlock::get();
		assert!(
			max_timeouts_per_block <= num_timeouts,
			"MaxHeartbeatTimeoutsPerBlock {max_timeouts_per_block} > max number of Heartbeat timeouts per block tested = {num_timeouts}"
		);
	}

	#[test]
	fn max_elections_per_block() {
		let avg_on_initialize: Weight =
			ON_INITIALIZE_BOUNDS.elections * (AVERAGE_ON_INITIALIZE_RATIO * MAXIMUM_BLOCK_WEIGHT);
		let try_elect_shard: Weight =
			<Runtime as pallet_elections::Config>::WeightInfo::try_elect_shards(1);
		assert!(
			try_elect_shard.all_lte(avg_on_initialize),
			"BUG: One shard election consumes more weight than available in on-initialize"
		);
		assert!(
			try_elect_shard
				.all_lte(<Runtime as pallet_elections::Config>::WeightInfo::try_elect_shards(2)),
			"BUG: 1 shard election consumes more weight than 2 shard elections"
		);
		let mut num_elections: u32 = 3;
		while <Runtime as pallet_elections::Config>::WeightInfo::try_elect_shards(num_elections)
			.all_lt(avg_on_initialize)
		{
			num_elections += 1;
			if num_elections == 10_000_000 {
				// 10_000_000 elections; halting to break out of loop
				break;
			}
		}
		let max_elections_per_block: u32 =
			<Runtime as pallet_elections::Config>::MaxElectionsPerBlock::get();
		assert!(
			max_elections_per_block <= num_elections,
			"MaxElectionsPerBlock {max_elections_per_block} > max number of Elections per block tested = {num_elections}"
		);
	}

	#[test]
	fn max_tasks_per_block() {
		let avg_on_initialize: Weight =
			ON_INITIALIZE_BOUNDS.tasks * (AVERAGE_ON_INITIALIZE_RATIO * MAXIMUM_BLOCK_WEIGHT);
		assert!(
			<Runtime as pallet_tasks::Config>::WeightInfo::schedule_tasks(1)
				.all_lte(avg_on_initialize),
			"BUG: Scheduling a single task consumes more weight than available in on-initialize"
		);
		assert!(
			<Runtime as pallet_tasks::Config>::WeightInfo::schedule_tasks(1)
				.all_lte(<Runtime as pallet_tasks::Config>::WeightInfo::schedule_tasks(2)),
			"BUG: Scheduling 1 task consumes more weight than scheduling 2"
		);
		let mut num_tasks: u32 = 2;
		while <Runtime as pallet_tasks::Config>::WeightInfo::schedule_tasks(num_tasks)
			.all_lt(avg_on_initialize)
		{
			num_tasks += 1;
			if num_tasks == 10_000_000 {
				// 10_000_000 tasks reached; halting to break out of loop
				break;
			}
		}
		let max_tasks_per_block_configured: u32 =
			<Runtime as pallet_tasks::Config>::MaxTasksPerBlock::get();
		assert!(
			max_tasks_per_block_configured <= num_tasks,
			"MaxTasksPerBlock {max_tasks_per_block_configured} > max number of tasks per block tested = {num_tasks}"
		);
	}
}
