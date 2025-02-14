use crate::{
	BatchIdCounter, BatchTaskId, Call, Config, FailedBatchIds, Pallet, ReadEventsTask,
	ShardRegistered, TaskIdCounter, TaskNetwork, TaskOutput, TaskShard,
};
use frame_benchmarking::benchmarks;
use frame_support::pallet_prelude::Get;
use frame_support::traits::OnInitialize;
use frame_system::RawOrigin;
use pallet_members::MemberPublicKey;
use pallet_networks::NetworkGatewayAddress;
use pallet_shards::{ShardCommitment, ShardState};
use polkadot_sdk::{frame_benchmarking, frame_support, frame_system, sp_core, sp_runtime, sp_std};
use sp_runtime::{BoundedVec, Vec};
use sp_std::vec;
use time_primitives::{
	AccountId, BatchId, Commitment, ElectionsInterface, ErrorMsg, GmpEvents, NetworkId, PublicKey,
	ShardStatus, ShardsInterface, Task, TaskId, TaskResult, TasksInterface, TssPublicKey,
	TssSignature,
};

const ETHEREUM: NetworkId = 0;
// Generated by running tests::bench_helper::print_valid_result
const PUBKEY: TssPublicKey = [
	2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160, 98, 149, 206, 135, 11, 7, 2, 155, 252, 219,
	45, 206, 40, 217, 89, 242, 129, 91, 22, 248, 23, 152,
];
const SIGNATURE: TssSignature = [
	230, 20, 79, 14, 121, 119, 238, 244, 64, 112, 113, 251, 64, 121, 242, 101, 133, 14, 213, 230,
	204, 138, 35, 254, 156, 112, 152, 250, 130, 8, 2, 140, 43, 187, 104, 130, 181, 13, 147, 50,
	199, 74, 169, 7, 176, 107, 129, 28, 186, 95, 143, 142, 159, 234, 44, 38, 160, 50, 47, 179, 194,
	78, 182, 197,
];

fn create_shard<
	T: Config + pallet_shards::Config + pallet_networks::Config + pallet_members::Config,
>(
	network: NetworkId,
) {
	NetworkGatewayAddress::<T>::insert(network, [0; 32]);
	let shard_id = <T as Config>::Shards::create_shard(
		network,
		[[0u8; 32].into(), [1u8; 32].into(), [2u8; 32].into()].to_vec(),
		1,
	)
	.unwrap_or_default();
	for m in [[0u8; 32], [1u8; 32], [2u8; 32]] {
		let pk = PublicKey::Sr25519(sp_core::sr25519::Public::from_raw(m));
		let acc: AccountId = m.into();
		MemberPublicKey::<T>::insert(acc, pk);
	}
	ShardCommitment::<T>::insert(shard_id, Commitment(BoundedVec::truncate_from(vec![PUBKEY])));
	ShardRegistered::<T>::insert(PUBKEY, ());
	Pallet::<T>::shard_online(shard_id, network);
	ShardState::<T>::insert(shard_id, ShardStatus::Online);
}

fn create_task<T: Config>(network: NetworkId) -> TaskId {
	Pallet::<T>::create_task(network, Task::ReadGatewayEvents { blocks: 0..10 })
}

benchmarks! {
	where_clause {  where T: pallet_shards::Config + pallet_networks::Config + pallet_members::Config }

	submit_task_result {
		create_shard::<T>(ETHEREUM);
		let task_id = create_task::<T>(ETHEREUM);
		Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		assert!(TaskShard::<T>::get(task_id).is_some());
	}: _(
		RawOrigin::Signed([0u8; 32].into()),
		task_id,
		TaskResult::ReadGatewayEvents { events: GmpEvents(vec![]), signature: SIGNATURE }
	) verify {
		assert_eq!(TaskOutput::<T>::get(task_id), Some(Ok(())));
		assert!(TaskShard::<T>::get(task_id).is_none());
	}

	schedule_tasks {
		let b in 1..<T as Config>::MaxTasksPerBlock::get();
		// reset storage from previous runs
		TaskIdCounter::<T>::take();
		let max_shards_created_per_block = <<T as pallet_shards::Config>::Elections as ElectionsInterface>::MaxElectionsPerBlock::get();
		for i in 0..b {
			let network: NetworkId = i.try_into().unwrap_or_default();
			create_shard::<T>(network);
			TaskShard::<T>::take(create_task::<T>(network));
			if i > 0 && i % max_shards_created_per_block == 0 {
				let mut now = frame_system::Pallet::<T>::block_number();
				now += 1u32.into();
				frame_system::Pallet::<T>::set_block_number(now);
				pallet_shards::Pallet::<T>::on_initialize(now);
			}
		}
		assert_eq!(TaskIdCounter::<T>::get(), b as u64);
	}: {
		Pallet::<T>::schedule_tasks();
	} verify {
		let mut unassigned = Vec::<u64>::new();
		for i in 0..b {
			let task_id: u64 = (i + 1).into();
			if TaskShard::<T>::get(task_id).is_none() {
				unassigned.push(task_id);
			}
		}
		assert_eq!(Vec::<u64>::new(), unassigned);
	}

	prepare_batches {
		let b in 1..<T as Config>::MaxBatchesPerBlock::get();
		// reset storage from previous runs
		BatchIdCounter::<T>::take();
		let max_shards_created_per_block = <<T as pallet_shards::Config>::Elections as ElectionsInterface>::MaxElectionsPerBlock::get();
		for i in 0..b {
			let network: NetworkId = i.try_into().unwrap_or_default();
			create_shard::<T>(network);
			create_task::<T>(network);
			if i > 0 && i % max_shards_created_per_block == 0 {
				let mut now = frame_system::Pallet::<T>::block_number();
				now += 1u32.into();
				frame_system::Pallet::<T>::set_block_number(now);
				pallet_shards::Pallet::<T>::on_initialize(now);
			}
		}
		assert_eq!(BatchIdCounter::<T>::get(), 0u64);
	}: {
		Pallet::<T>::prepare_batches();
	} verify {
		assert_eq!(BatchIdCounter::<T>::get(), b as u64);
	}

	submit_gmp_events {
	}: _(RawOrigin::Root, ETHEREUM, GmpEvents(vec![]))
	verify { }

	sync_network {}: _(RawOrigin::Root, ETHEREUM, 100u64) verify { }

	stop_network {
		ReadEventsTask::<T>::insert(ETHEREUM, 1);
	}: _(RawOrigin::Root, ETHEREUM) verify { }

	remove_task {
		let task_id = create_task::<T>(ETHEREUM);
		TaskOutput::<T>::insert(task_id, Ok::<(), ErrorMsg>(()));
	}: _(RawOrigin::Root, task_id) verify { }


	restart_batch {
		let l in 1..100;
		create_shard::<T>(ETHEREUM);
		let network = ETHEREUM;
		let failed_batches: Vec<BatchId> = (0..l).map(|l| l as BatchId).collect();
		let target_batch_id: u64 = (l - 1).into();

		for batch_id in &failed_batches {
			let task_id = Pallet::<T>::create_task(
				network,
				Task::SubmitGatewayMessage { batch_id: *batch_id }
			);
			BatchTaskId::<T>::insert(batch_id, task_id);
			TaskNetwork::<T>::insert(task_id, network);
		}

		FailedBatchIds::<T>::put(failed_batches.clone());
	}: _(RawOrigin::Root, target_batch_id) verify {
		let new_task_id = TaskIdCounter::<T>::get();
		assert_eq!(
			BatchTaskId::<T>::get(target_batch_id),
			Some(new_task_id),
			"New task not created"
		);
		assert!(
			!FailedBatchIds::<T>::get().contains(&target_batch_id),
			"Batch not removed from failed list"
		);
		assert_eq!(
			FailedBatchIds::<T>::get().len(),
			(l - 1) as usize,
			"List length mismatch"
		);

	   }
	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
