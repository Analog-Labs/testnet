//! Runtime Benchmarks List.
//!
//! Provides available benchmarks via [`add_benchmarks`] and [`list_benchmarks`]

polkadot_sdk::frame_benchmarking::define_benchmarks!(
	[frame_benchmarking, BaselineBench::<Runtime>]
	[frame_system, SystemBench::<Runtime>]
	[pallet_babe, Babe]
	[pallet_bags_list, VoterList]
	[pallet_balances, Balances]
	[pallet_collective, TechnicalCommittee]
	[pallet_elections, Elections]
	[pallet_dmail, Dmail]
	[pallet_election_provider_multi_phase, ElectionProviderMultiPhase]
	[pallet_election_provider_support_benchmarking, EPSBench::<Runtime>]
	[pallet_grandpa, Grandpa]
	[pallet_identity, Identity]
	[pallet_im_online, ImOnline]
	[pallet_membership, TechnicalMembership]
	[pallet_members, Members]
	[pallet_multisig, Multisig]
	[pallet_networks, Networks]
	[pallet_offences, OffencesBench::<Runtime>]
	[pallet_preimage, Preimage]
	[pallet_proxy, Proxy]
	[pallet_scheduler, Scheduler]
	[pallet_session, SessionBench::<Runtime>]
	[pallet_shards, Shards]
	[pallet_staking, Staking]
	[pallet_tasks, Tasks]
	[pallet_timegraph, Timegraph]
	[pallet_timestamp, Timestamp]
	[pallet_treasury, Treasury]
	[pallet_utility, Utility]
	[pallet_vesting, Vesting]
);
