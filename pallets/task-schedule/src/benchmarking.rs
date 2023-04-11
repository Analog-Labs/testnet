use super::*;
#[allow(unused)]
use crate::Pallet as TaskSchedule;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use scale_info::prelude::string::String;
use time_primitives::abstraction::{
	ObjectId, ScheduleInput as Schedule, ScheduleStatus, TaskSchedule as TaskScheduleInput,
	Validity,
};

benchmarks! {

	insert_schedule {
		let origin: T::AccountId = whitelisted_caller();
		let input  = Schedule {
			task_id: ObjectId(1),
			shard_id: 1,
			cycle: 12,
			validity: Validity::Seconds(10),
			hash:String::from("address"),
		};

		let schedule = input.clone();
	}: _(RawOrigin::Signed(origin), schedule)
	verify {
		assert!( <ScheduleStorage<T>>::get(1).is_some());
	}

	update_schedule {
		let origin: T::AccountId = whitelisted_caller();
		let input  = TaskScheduleInput {
			task_id: ObjectId(1),
			owner:origin.clone(),
			shard_id: 1,
			cycle: 12,
			validity: Validity::Seconds(10),
			hash:String::from("address"),
			status: ScheduleStatus::Initiated,
		};

		<ScheduleStorage<T>>::insert(
			1,
			input
		);
	}: _(RawOrigin::Signed(origin), ScheduleStatus::Completed, 1)
	verify {
		assert!( <ScheduleStorage<T>>::get(1).is_some());
	}

	impl_benchmark_test_suite!(TaskSchedule, crate::mock::new_test_ext(), crate::mock::Test);
}
