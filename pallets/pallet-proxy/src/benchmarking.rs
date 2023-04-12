use super::*;
#[allow(unused)]
use crate::Pallet as PalletProxy;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::SaturatedConversion;
use time_primitives::{ProxyAccInput, ProxyAccStatus, ProxyStatus};
benchmarks! {

	set_proxy_account {
		let origin: T::AccountId = whitelisted_caller();
		let proxy_acc: T::AccountId = whitelisted_caller();
		let input = ProxyAccInput {
			max_token_usage: 10u32,
			token_usage: 10u32,
			max_task_execution: Some(10u32),
			task_executed: 10u32,
			proxy: proxy_acc,
		};

	}: _(RawOrigin::Signed(origin.clone()), input)
	verify {
		assert!( <ProxyStorage<T>>::get(origin).is_some());
	}

	update_proxy_account {
		let origin: T::AccountId = whitelisted_caller();
		let proxy_acc: T::AccountId = whitelisted_caller();
		let input = ProxyAccInput {
			max_token_usage: 10u32,
			token_usage: 10u32,
			max_task_execution: Some(10u32),
			task_executed: 10u32,
			proxy: proxy_acc,
		};

		let data = ProxyAccStatus {
			owner: origin.clone(),
			max_token_usage: input.max_token_usage.saturated_into(),
			token_usage: input.token_usage.saturated_into(),
			max_task_execution: input.max_task_execution,
			task_executed: input.task_executed,
			status: ProxyStatus::Valid,
			proxy: input.proxy,
		};
		let _ = <ProxyStorage<T>>::insert(origin.clone(),data);

	}: _(RawOrigin::Signed(origin.clone()), ProxyStatus::Suspended)
	verify {
		assert!( <ProxyStorage<T>>::get(origin).is_some());
	}

	remove_proxy_account {
		let origin: T::AccountId = whitelisted_caller();
		let proxy_acc: T::AccountId = whitelisted_caller();
		let input = ProxyAccInput {
			max_token_usage: 10u32,
			token_usage: 10u32,
			max_task_execution: Some(10u32),
			task_executed: 10u32,
			proxy: proxy_acc,
		};

		let data = ProxyAccStatus {
			owner: origin.clone(),
			max_token_usage: input.max_token_usage.saturated_into(),
			token_usage: input.token_usage.saturated_into(),
			max_task_execution: input.max_task_execution,
			task_executed: input.task_executed,
			status: ProxyStatus::Valid,
			proxy: input.proxy.clone(),
		};
		let _ = <ProxyStorage<T>>::insert(origin.clone(),data);

	}: _(RawOrigin::Signed(origin.clone()), input.proxy)
	verify {
		assert!( <ProxyStorage<T>>::get(origin).is_some());
	}

	impl_benchmark_test_suite!(PalletProxy, crate::mock::new_test_ext(), crate::mock::Test);
}
