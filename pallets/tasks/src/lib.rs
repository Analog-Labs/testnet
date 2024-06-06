#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub use pallet::*;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use core::num::NonZeroU64;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::string::String;
	use sp_runtime::{
		traits::{AccountIdConversion, IdentifyAccount, Zero},
		Saturating,
	};
	use sp_std::vec;
	use sp_std::vec::Vec;
	use time_primitives::{
		AccountId, Balance, DepreciationRate, ElectionsInterface, Function, GmpParams, Message,
		Msg, NetworkId, Payload, PublicKey, RewardConfig, ShardId, ShardsInterface, TaskDescriptor,
		TaskDescriptorParams, TaskExecution, TaskFunder, TaskId, TaskPhase, TaskResult,
		TasksInterface, TransferStake, TssSignature,
	};

	pub trait WeightInfo {
		fn create_task(input_length: u32) -> Weight;
		fn submit_result(input_length: u32) -> Weight;
		fn submit_hash() -> Weight;
		fn submit_signature() -> Weight;
		fn register_gateway() -> Weight;
		fn set_read_task_reward() -> Weight;
		fn set_write_task_reward() -> Weight;
		fn set_send_message_task_reward() -> Weight;
		fn sudo_cancel_task() -> Weight;
		fn sudo_cancel_tasks(n: u32) -> Weight;
		fn reset_tasks(n: u32) -> Weight;
		fn set_shard_task_limit() -> Weight;
		fn unregister_gateways(n: u32) -> Weight;
		fn set_batch_size() -> Weight;
	}

	impl WeightInfo for () {
		fn create_task(_: u32) -> Weight {
			Weight::default()
		}

		fn submit_result(_: u32) -> Weight {
			Weight::default()
		}

		fn submit_hash() -> Weight {
			Weight::default()
		}

		fn submit_signature() -> Weight {
			Weight::default()
		}

		fn register_gateway() -> Weight {
			Weight::default()
		}

		fn set_read_task_reward() -> Weight {
			Weight::default()
		}

		fn set_write_task_reward() -> Weight {
			Weight::default()
		}

		fn set_send_message_task_reward() -> Weight {
			Weight::default()
		}

		fn sudo_cancel_task() -> Weight {
			Weight::default()
		}

		fn sudo_cancel_tasks(_: u32) -> Weight {
			Weight::default()
		}

		fn reset_tasks(_: u32) -> Weight {
			Weight::default()
		}

		fn set_shard_task_limit() -> Weight {
			Weight::default()
		}

		fn unregister_gateways(_: u32) -> Weight {
			Weight::default()
		}

		fn set_batch_size() -> Weight {
			Weight::default()
		}
	}

	type BalanceOf<T> = <T as pallet_balances::Config>::Balance;

	const BATCH_SIZE: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(32) };

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config<AccountId = AccountId> + pallet_balances::Config<Balance = Balance>
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type Shards: ShardsInterface;
		type Elections: ElectionsInterface;
		type Members: TransferStake;
		/// Base read task reward (for all networks)
		#[pallet::constant]
		type BaseReadReward: Get<BalanceOf<Self>>;
		/// Base write task reward (for all networks)
		#[pallet::constant]
		type BaseWriteReward: Get<BalanceOf<Self>>;
		/// Base send message task reward (for all networks)
		#[pallet::constant]
		type BaseSendMessageReward: Get<BalanceOf<Self>>;
		/// Reward declines every N blocks since read task start by Percent * Amount
		#[pallet::constant]
		type RewardDeclineRate: Get<DepreciationRate<BlockNumberFor<Self>>>;
		#[pallet::constant]
		type SignPhaseTimeout: Get<BlockNumberFor<Self>>;
		#[pallet::constant]
		type WritePhaseTimeout: Get<BlockNumberFor<Self>>;
		#[pallet::constant]
		type ReadPhaseTimeout: Get<BlockNumberFor<Self>>;
		/// `PalletId` for this pallet, used to derive an account for each task.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::storage]
	pub type UnassignedTasks<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		NetworkId,
		Blake2_128Concat,
		u64,
		TaskId,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type UATasksInsertIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, u64, OptionQuery>;

	#[pallet::storage]
	pub type UATasksRemoveIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, u64, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn shard_task_limit)]
	pub type ShardTaskLimit<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, u32, OptionQuery>;

	#[pallet::storage]
	pub type ShardTasks<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, ShardId, Blake2_128Concat, TaskId, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task_shard)]
	pub type TaskShard<T: Config> = StorageMap<_, Blake2_128Concat, TaskId, ShardId, OptionQuery>;

	#[pallet::storage]
	pub type NetworkShards<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		NetworkId,
		Blake2_128Concat,
		ShardId,
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	pub type NetworkBatchSize<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, u64, OptionQuery>;

	#[pallet::storage]
	pub type NetworkOffset<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, u64, OptionQuery>;

	#[pallet::storage]
	pub type TaskIdCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	pub type Tasks<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, TaskDescriptor, OptionQuery>;

	#[pallet::storage]
	pub type TaskPhaseState<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, TaskPhase, ValueQuery>;

	#[pallet::storage]
	pub type TaskSignature<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, TssSignature, OptionQuery>;

	#[pallet::storage]
	pub type TaskSigner<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, PublicKey, OptionQuery>;

	#[pallet::storage]
	pub type TaskHash<T: Config> = StorageMap<_, Blake2_128Concat, TaskId, [u8; 32], OptionQuery>;

	#[pallet::storage]
	pub type PhaseStart<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		TaskId,
		Blake2_128Concat,
		TaskPhase,
		BlockNumberFor<T>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub type TaskOutput<T: Config> =
		StorageMap<_, Blake2_128Concat, TaskId, TaskResult, OptionQuery>;

	#[pallet::storage]
	pub type ShardRegistered<T: Config> = StorageMap<_, Blake2_128Concat, ShardId, (), OptionQuery>;

	#[pallet::storage]
	pub type Gateway<T: Config> = StorageMap<_, Blake2_128Concat, NetworkId, [u8; 20], OptionQuery>;

	#[pallet::storage]
	pub type RecvTasks<T: Config> = StorageMap<_, Blake2_128Concat, NetworkId, u64, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn network_read_reward)]
	pub type NetworkReadReward<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn network_write_reward)]
	pub type NetworkWriteReward<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn network_send_message_reward)]
	pub type NetworkSendMessageReward<T: Config> =
		StorageMap<_, Blake2_128Concat, NetworkId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task_reward_config)]
	pub type TaskRewardConfig<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TaskId,
		RewardConfig<BalanceOf<T>, BlockNumberFor<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn signer_payout)]
	pub type SignerPayout<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		TaskId,
		Blake2_128Concat,
		AccountId,
		BalanceOf<T>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub type MessageTask<T: Config> =
		StorageMap<_, Blake2_128Concat, [u8; 32], (TaskId, TaskId), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// the record id that uniquely identify
		TaskCreated(TaskId),
		/// Task succeeded with result
		TaskResult(TaskId, TaskResult),
		/// Gateway registered on network
		GatewayRegistered(NetworkId, [u8; 20], u64),
		/// Read task reward set for network
		ReadTaskRewardSet(NetworkId, BalanceOf<T>),
		/// Write task reward set for network
		WriteTaskRewardSet(NetworkId, BalanceOf<T>),
		/// Send message task reward set for network
		SendMessageTaskRewardSet(NetworkId, BalanceOf<T>),
		/// Set the maximum number of assigned tasks for all shards on the network
		ShardTaskLimitSet(NetworkId, u32),
		/// Set the network batch size
		BatchSizeSet(NetworkId, u64, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unknown Task
		UnknownTask,
		/// Unknown Shard
		UnknownShard,
		/// Invalid Signature
		InvalidSignature,
		/// Signature Verification Failed
		SignatureVerificationFailed,
		/// Invalid Owner
		InvalidOwner,
		/// Invalid task function
		InvalidTaskFunction,
		/// Not sign phase
		NotSignPhase,
		/// Not write phase
		NotWritePhase,
		/// Not read phase
		NotReadPhase,
		/// Invalid signer
		InvalidSigner,
		/// Task not assigned
		UnassignedTask,
		/// Task already signed
		TaskSigned,
		/// Cannot submit result for GMP functions unless gateway is registered
		GatewayNotRegistered,
		/// Bootstrap shard must be online to call register_gateway
		BootstrapShardMustBeOnline,
		/// Shard for task must be online at task creation
		MatchingShardNotOnline,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_task(schedule.function.get_input_length()))]
		pub fn create_task(origin: OriginFor<T>, schedule: TaskDescriptorParams) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				T::Shards::matching_shard_online(schedule.network, schedule.shard_size),
				Error::<T>::MatchingShardNotOnline
			);
			Self::start_task(schedule, TaskFunder::Account(who))?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::submit_result(result.payload.get_input_length()))]
		pub fn submit_result(
			origin: OriginFor<T>,
			task_id: TaskId,
			result: TaskResult,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let task = Tasks::<T>::get(task_id).ok_or(Error::<T>::UnknownTask)?;
			if TaskOutput::<T>::get(task_id).is_some() {
				return Ok(());
			}
			ensure!(TaskPhaseState::<T>::get(task_id) == TaskPhase::Read, Error::<T>::NotReadPhase);
			let shard_id = TaskShard::<T>::get(task_id).ok_or(Error::<T>::UnassignedTask)?;
			ensure!(result.shard_id == shard_id, Error::<T>::InvalidOwner);
			let bytes = result.payload.bytes(task_id);
			Self::validate_signature(result.shard_id, &bytes, result.signature)?;
			Self::finish_task(task_id, result.clone());
			Self::payout_task_rewards(task_id, result.shard_id, task.function.initial_phase());
			if let Function::RegisterShard { shard_id } = task.function {
				ShardRegistered::<T>::insert(shard_id, ());
			}
			if let Payload::Gmp(msgs) = &result.payload {
				for msg in msgs {
					let send_task_id = Self::send_message(result.shard_id, msg.clone());
					MessageTask::<T>::insert(msg.salt, (task_id, send_task_id));
				}
				if let Some(block_height) = RecvTasks::<T>::get(task.network) {
					let batch_size = NetworkBatchSize::<T>::get(task.network)
						.and_then(NonZeroU64::new)
						.unwrap_or(BATCH_SIZE);
					if let Some(next_block_height) = block_height.checked_add(batch_size.into()) {
						Self::recv_messages(task.network, next_block_height, batch_size);
					}
				}
			}
			Self::deposit_event(Event::TaskResult(task_id, result));
			Self::schedule_tasks(task.network, Some(shard_id));
			Ok(())
		}

		/// Submit Task Hash
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::submit_hash())]
		pub fn submit_hash(
			origin: OriginFor<T>,
			task_id: TaskId,
			hash: Result<[u8; 32], String>,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(Tasks::<T>::get(task_id).is_some(), Error::<T>::UnknownTask);
			ensure!(
				TaskPhaseState::<T>::get(task_id) == TaskPhase::Write,
				Error::<T>::NotWritePhase
			);
			let expected_signer = TaskSigner::<T>::get(task_id);
			ensure!(
				Some(signer.clone()) == expected_signer.map(|s| s.into_account()),
				Error::<T>::InvalidSigner
			);
			let shard_id = TaskShard::<T>::get(task_id).ok_or(Error::<T>::UnassignedTask)?;
			Self::snapshot_write_reward(task_id, signer);
			match hash {
				Ok(hash) => {
					TaskHash::<T>::insert(task_id, hash);
					Self::start_phase(shard_id, task_id, TaskPhase::Read);
				},
				Err(err) => {
					Self::finish_task(
						task_id,
						TaskResult {
							shard_id,
							payload: Payload::Error(err),
							signature: [0; 64],
						},
					);
				},
			}
			Ok(())
		}

		/// Submit Signature
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::submit_signature())]
		pub fn submit_signature(
			origin: OriginFor<T>,
			task_id: TaskId,
			signature: TssSignature,
		) -> DispatchResult {
			ensure_signed(origin)?;
			ensure!(TaskSignature::<T>::get(task_id).is_none(), Error::<T>::TaskSigned);
			ensure!(Tasks::<T>::get(task_id).is_some(), Error::<T>::UnknownTask);
			ensure!(TaskPhaseState::<T>::get(task_id) == TaskPhase::Sign, Error::<T>::NotSignPhase);
			let Some(shard_id) = TaskShard::<T>::get(task_id) else {
				return Err(Error::<T>::UnassignedTask.into());
			};
			let bytes = Self::get_gmp_hash(task_id, shard_id)?;
			Self::validate_signature(shard_id, &bytes, signature)?;
			Self::start_phase(shard_id, task_id, TaskPhase::Write);
			TaskSignature::<T>::insert(task_id, signature);
			Ok(())
		}

		/// Register gateway
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::register_gateway())]
		pub fn register_gateway(
			origin: OriginFor<T>,
			bootstrap: ShardId,
			address: [u8; 20],
			block_height: u64,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(T::Shards::is_shard_online(bootstrap), Error::<T>::BootstrapShardMustBeOnline);
			let network = T::Shards::shard_network(bootstrap).ok_or(Error::<T>::UnknownShard)?;
			for (shard_id, _) in NetworkShards::<T>::iter_prefix(network) {
				Self::unregister_shard(shard_id, network);
			}
			ShardRegistered::<T>::insert(bootstrap, ());
			for (shard_id, _) in NetworkShards::<T>::iter_prefix(network) {
				if shard_id != bootstrap {
					Self::register_shard(shard_id, network);
				}
			}
			let gateway_changed = Gateway::<T>::get(network).is_some();
			Gateway::<T>::insert(network, address);
			Self::deposit_event(Event::GatewayRegistered(network, address, block_height));
			if !gateway_changed {
				let network_batch_size = NetworkBatchSize::<T>::get(network)
					.and_then(NonZeroU64::new)
					.unwrap_or(BATCH_SIZE);
				let network_offset = NetworkOffset::<T>::get(network).unwrap_or(0);
				let batch_size = NonZeroU64::new(network_batch_size.get() - ((block_height + network_offset) % network_batch_size))
					.expect("x = block_height % BATCH_SIZE ==> x <= BATCH_SIZE - 1 ==> BATCH_SIZE - x >= 1; QED");
				let block_height = block_height + batch_size.get();
				Self::recv_messages(network, block_height, batch_size);
			}
			Self::schedule_tasks(network, None);
			Ok(())
		}

		/// Set read task reward
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::set_read_task_reward())]
		pub fn set_read_task_reward(
			origin: OriginFor<T>,
			network: NetworkId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			NetworkReadReward::<T>::insert(network, amount);
			Self::deposit_event(Event::ReadTaskRewardSet(network, amount));
			Ok(())
		}

		/// Set write task reward
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::set_write_task_reward())]
		pub fn set_write_task_reward(
			origin: OriginFor<T>,
			network: NetworkId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			NetworkWriteReward::<T>::insert(network, amount);
			Self::deposit_event(Event::WriteTaskRewardSet(network, amount));
			Ok(())
		}

		/// Set send message task reward
		#[pallet::call_index(7)]
		#[pallet::weight(<T as Config>::WeightInfo::set_send_message_task_reward())]
		pub fn set_send_message_task_reward(
			origin: OriginFor<T>,
			network: NetworkId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			NetworkSendMessageReward::<T>::insert(network, amount);
			Self::deposit_event(Event::SendMessageTaskRewardSet(network, amount));
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(<T as Config>::WeightInfo::sudo_cancel_task())]
		pub fn sudo_cancel_task(origin: OriginFor<T>, task_id: TaskId) -> DispatchResult {
			ensure_root(origin)?;
			let task = Tasks::<T>::get(task_id).ok_or(Error::<T>::UnknownTask)?;
			Self::cancel_task(task_id, task.network);
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(<T as Config>::WeightInfo::sudo_cancel_tasks(*max))]
		pub fn sudo_cancel_tasks(origin: OriginFor<T>, max: u32) -> DispatchResult {
			ensure_root(origin)?;
			// TODO (followup): ensure max <= PalletMax which is set according to our current block size limit
			let mut cancelled = 0;
			for (network, _, task_id) in UnassignedTasks::<T>::iter() {
				if cancelled >= max {
					return Ok(());
				}
				Self::cancel_task(task_id, network);
				cancelled = cancelled.saturating_plus_one();
			}
			for (shard_id, task_id, _) in ShardTasks::<T>::iter() {
				if let Some(network) = T::Shards::shard_network(shard_id) {
					if cancelled >= max {
						return Ok(());
					}
					Self::cancel_task(task_id, network);
					cancelled = cancelled.saturating_plus_one();
				}
			}
			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(<T as Config>::WeightInfo::reset_tasks(*max))]
		pub fn reset_tasks(origin: OriginFor<T>, max: u32) -> DispatchResult {
			ensure_root(origin)?;
			let mut to_be_reset = 0u32;
			for (task_id, shard_id) in TaskShard::<T>::drain() {
				ShardTasks::<T>::remove(shard_id, task_id);
				if let Some(task) = Tasks::<T>::get(task_id) {
					if to_be_reset >= max {
						break;
					}
					Self::add_unassigned_task(task.network, task_id);
					to_be_reset = to_be_reset.saturating_plus_one();
				}
			}
			let mut reset = 0u32;
			for (_, _, task_id) in UnassignedTasks::<T>::iter() {
				if let Some(task) = Tasks::<T>::get(task_id) {
					if reset >= max {
						break;
					}
					TaskPhaseState::<T>::insert(task_id, task.function.initial_phase());
					reset = reset.saturating_plus_one();
				}
			}
			for (network, shard, _) in NetworkShards::<T>::iter() {
				Self::schedule_tasks(network, Some(shard));
			}
			Ok(())
		}

		#[pallet::call_index(11)]
		#[pallet::weight(<T as Config>::WeightInfo::set_shard_task_limit())]
		pub fn set_shard_task_limit(
			origin: OriginFor<T>,
			network: NetworkId,
			limit: u32,
		) -> DispatchResult {
			ensure_root(origin)?;
			ShardTaskLimit::<T>::insert(network, limit);
			Self::deposit_event(Event::ShardTaskLimitSet(network, limit));
			Ok(())
		}

		#[pallet::call_index(12)]
		#[pallet::weight(<T as Config>::WeightInfo::unregister_gateways(*num_gateways))]
		pub fn unregister_gateways(origin: OriginFor<T>, num_gateways: u32) -> DispatchResult {
			ensure_root(origin)?;
			let _ = Gateway::<T>::clear(num_gateways, None);
			// safest to keep this clear_all or add additional weight hint in
			// follow up. Iterating through only the removed gateways would complicate
			// things
			let _ = ShardRegistered::<T>::clear(u32::MAX, None);
			Self::filter_tasks(|task_id| {
				let Some(task) = Tasks::<T>::get(task_id) else {
					return;
				};
				if let Function::ReadMessages { .. } = task.function {
					Self::finish_task(
						task_id,
						TaskResult {
							shard_id: 0,
							payload: Payload::Error("shard offline or gateway changed".into()),
							signature: [0u8; 64],
						},
					);
				}
			});
			Ok(())
		}

		#[pallet::call_index(13)]
		#[pallet::weight(<T as Config>::WeightInfo::set_batch_size())]
		pub fn set_batch_size(
			origin: OriginFor<T>,
			network: NetworkId,
			batch_size: u64,
			offset: u64,
		) -> DispatchResult {
			ensure_root(origin)?;
			NetworkBatchSize::<T>::insert(network, batch_size);
			NetworkOffset::<T>::insert(network, offset);
			Self::deposit_event(Event::BatchSizeSet(network, batch_size, offset));
			Ok(())
		}
	}

	/*#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(current: BlockNumberFor<T>) -> Weight {
			let mut writes = 0;
			TaskShard::<T>::iter().for_each(|(task_id, shard_id)| {
				let phase = TaskPhaseState::<T>::get(task_id);
				let start = PhaseStart::<T>::get(task_id, phase);
				let timeout = match phase {
					TaskPhase::Sign => T::SignPhaseTimeout::get(),
					TaskPhase::Write => T::WritePhaseTimeout::get(),
					TaskPhase::Read => T::ReadPhaseTimeout::get(),
				};
				if current.saturating_sub(start) >= timeout {
					if phase == TaskPhase::Write {
						Self::start_phase(shard_id, task_id, phase);
					} else {
						Self::schedule_task(task_id);
					}
					writes += 3;
				}
			});
			T::DbWeight::get().writes(writes)
		}
	}*/

	impl<T: Config> Pallet<T> {
		pub fn get_task_signature(task: TaskId) -> Option<TssSignature> {
			TaskSignature::<T>::get(task)
		}

		pub fn get_task_signer(task: TaskId) -> Option<PublicKey> {
			TaskSigner::<T>::get(task)
		}

		pub fn get_task_hash(task: TaskId) -> Option<[u8; 32]> {
			TaskHash::<T>::get(task)
		}

		pub fn get_shard_tasks(shard_id: ShardId) -> Vec<TaskExecution> {
			ShardTasks::<T>::iter_prefix(shard_id)
				.map(|(task_id, _)| TaskExecution::new(task_id, TaskPhaseState::<T>::get(task_id)))
				.collect()
		}

		pub fn get_task(task_id: TaskId) -> Option<TaskDescriptor> {
			Tasks::<T>::get(task_id)
		}

		pub fn get_gateway(network: NetworkId) -> Option<[u8; 20]> {
			Gateway::<T>::get(network)
		}

		pub fn get_task_phase(task_id: TaskId) -> TaskPhase {
			TaskPhaseState::<T>::get(task_id)
		}

		pub fn get_task_shard(task_id: TaskId) -> Option<ShardId> {
			TaskShard::<T>::get(task_id)
		}

		pub fn get_task_result(task_id: TaskId) -> Option<TaskResult> {
			TaskOutput::<T>::get(task_id)
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID containing the funds for a task.
		fn task_account(task_id: TaskId) -> T::AccountId {
			T::PalletId::get().into_sub_account_truncating(task_id)
		}

		fn recv_messages(network_id: NetworkId, block_height: u64, batch_size: NonZeroU64) {
			RecvTasks::<T>::insert(network_id, block_height);
			let mut task = TaskDescriptorParams::new(
				network_id,
				Function::ReadMessages { batch_size },
				T::Elections::default_shard_size(),
			);
			task.start = block_height;
			Self::start_task(task, TaskFunder::Inflation).expect("task funded through inflation");
		}

		fn register_shard(shard_id: ShardId, network_id: NetworkId) {
			Self::start_task(
				TaskDescriptorParams::new(
					network_id,
					Function::RegisterShard { shard_id },
					T::Shards::shard_members(shard_id).len() as _,
				),
				TaskFunder::Inflation,
			)
			.expect("task funded through inflation");
		}

		fn filter_tasks<F: Fn(TaskId)>(f: F) {
			for (_network, _, task_id) in UnassignedTasks::<T>::iter() {
				f(task_id);
			}
			for (_shard_id, task_id, _) in ShardTasks::<T>::iter() {
				f(task_id);
			}
		}

		fn unregister_shard(shard_id: ShardId, network: NetworkId) {
			if ShardRegistered::<T>::take(shard_id).is_some() {
				Self::start_task(
					TaskDescriptorParams::new(
						network,
						Function::UnregisterShard { shard_id },
						T::Shards::shard_members(shard_id).len() as _,
					),
					TaskFunder::Inflation,
				)
				.expect("task funded through inflation");
				return;
			}
			Self::filter_tasks(|task_id| {
				let Some(task) = Tasks::<T>::get(task_id) else {
					return;
				};
				if let Function::RegisterShard { shard_id: s } = task.function {
					if s == shard_id {
						Self::finish_task(
							task_id,
							TaskResult {
								shard_id: 0,
								payload: Payload::Error("shard offline or gateway changed".into()),
								signature: [0u8; 64],
							},
						);
					}
				}
			});
		}

		fn send_message(shard_id: ShardId, msg: Msg) -> TaskId {
			Self::start_task(
				TaskDescriptorParams::new(
					msg.dest_network,
					Function::SendMessage { msg },
					T::Shards::shard_members(shard_id).len() as _,
				),
				TaskFunder::Inflation,
			)
			.expect("task funded through inflation")
		}

		/// Start task
		fn start_task(
			schedule: TaskDescriptorParams,
			who: TaskFunder,
		) -> Result<TaskId, DispatchError> {
			let task_id = TaskIdCounter::<T>::get();
			let phase = schedule.function.initial_phase();
			let (read_task_reward, write_task_reward, send_message_reward) = (
				T::BaseReadReward::get() + NetworkReadReward::<T>::get(schedule.network),
				T::BaseWriteReward::get() + NetworkWriteReward::<T>::get(schedule.network),
				T::BaseSendMessageReward::get()
					+ NetworkSendMessageReward::<T>::get(schedule.network),
			);
			let mut required_funds = read_task_reward.saturating_mul(schedule.shard_size.into());
			if phase == TaskPhase::Write || phase == TaskPhase::Sign {
				required_funds = required_funds.saturating_add(write_task_reward);
			}
			if phase == TaskPhase::Sign {
				required_funds = required_funds
					.saturating_add(send_message_reward.saturating_mul(schedule.shard_size.into()));
			}
			let owner = match who {
				TaskFunder::Account(user) => {
					let funds = if schedule.funds >= required_funds {
						schedule.funds
					} else {
						required_funds
					};
					pallet_balances::Pallet::<T>::transfer(
						&user,
						&Self::task_account(task_id),
						funds,
						ExistenceRequirement::KeepAlive,
					)?;
					Some(user)
				},
				TaskFunder::Shard(shard_id) => {
					let task_account = Self::task_account(task_id);
					let amount = required_funds.saturating_div(schedule.shard_size.into());
					for member in T::Shards::shard_members(shard_id) {
						T::Members::transfer_stake(&member, &task_account, amount)?;
					}
					None
				},
				TaskFunder::Inflation => {
					pallet_balances::Pallet::<T>::resolve_creating(
						&Self::task_account(task_id),
						pallet_balances::Pallet::<T>::issue(required_funds),
					);
					None
				},
			};
			TaskRewardConfig::<T>::insert(
				task_id,
				RewardConfig {
					read_task_reward,
					write_task_reward,
					send_message_reward,
					depreciation_rate: T::RewardDeclineRate::get(),
				},
			);
			Tasks::<T>::insert(
				task_id,
				TaskDescriptor {
					owner,
					network: schedule.network,
					function: schedule.function,
					start: schedule.start,
					shard_size: schedule.shard_size,
				},
			);
			TaskPhaseState::<T>::insert(task_id, phase);
			TaskIdCounter::<T>::put(task_id.saturating_plus_one());
			Self::add_unassigned_task(schedule.network, task_id);
			Self::deposit_event(Event::TaskCreated(task_id));
			Self::schedule_tasks(schedule.network, None);
			Ok(task_id)
		}

		fn start_phase(shard_id: ShardId, task_id: TaskId, phase: TaskPhase) {
			let block = frame_system::Pallet::<T>::block_number();
			TaskPhaseState::<T>::insert(task_id, phase);
			PhaseStart::<T>::insert(task_id, phase, block);
			if phase == TaskPhase::Write {
				TaskSigner::<T>::insert(task_id, T::Shards::next_signer(shard_id));
			}
		}

		fn finish_task(task_id: TaskId, result: TaskResult) {
			TaskOutput::<T>::insert(task_id, result);
			if let Some(shard_id) = TaskShard::<T>::take(task_id) {
				ShardTasks::<T>::remove(shard_id, task_id);
			}
		}

		fn cancel_task(task_id: TaskId, task_network: NetworkId) {
			let result = TaskResult {
				shard_id: 0,
				payload: Payload::Error("task cancelled by sudo".into()),
				signature: [0; 64],
			};
			Self::finish_task(task_id, result.clone());
			Self::add_unassigned_task(task_network, task_id);
			TaskPhaseState::<T>::remove(task_id);
			TaskSigner::<T>::remove(task_id);
			TaskSignature::<T>::remove(task_id);
			TaskHash::<T>::remove(task_id);
			Self::deposit_event(Event::TaskResult(task_id, result));
		}

		fn validate_signature(
			shard_id: ShardId,
			data: &[u8],
			signature: TssSignature,
		) -> DispatchResult {
			let public_key = T::Shards::tss_public_key(shard_id).ok_or(Error::<T>::UnknownShard)?;
			let signature = schnorr_evm::Signature::from_bytes(signature)
				.map_err(|_| Error::<T>::InvalidSignature)?;
			let schnorr_public_key = schnorr_evm::VerifyingKey::from_bytes(public_key)
				.map_err(|_| Error::<T>::UnknownShard)?;
			schnorr_public_key
				.verify(data, &signature)
				.map_err(|_| Error::<T>::SignatureVerificationFailed)?;
			Ok(())
		}

		fn schedule_tasks(network: NetworkId, shard_id: Option<ShardId>) {
			if let Some(shard_id) = shard_id {
				Self::schedule_tasks_shard(network, shard_id);
			} else {
				for (shard, _) in NetworkShards::<T>::iter_prefix(network) {
					Self::schedule_tasks_shard(network, shard);
				}
			}
		}

		fn schedule_tasks_shard(network: NetworkId, shard_id: ShardId) {
			let tasks = ShardTasks::<T>::iter_prefix(shard_id)
				.filter(|(t, _)| TaskOutput::<T>::get(t).is_none())
				.count();
			let shard_size = T::Shards::shard_members(shard_id).len() as u16;
			let is_registered = ShardRegistered::<T>::get(shard_id).is_some();
			let shard_task_limit = ShardTaskLimit::<T>::get(network).unwrap_or(10) as usize;
			let capacity = shard_task_limit.saturating_sub(tasks);
			if capacity.is_zero() {
				// no new tasks assigned if capacity reached or exceeded
				return;
			}

			let insert_index = <UATasksInsertIndex<T>>::get(network).unwrap_or(0);
			let remove_index = <UATasksRemoveIndex<T>>::get(network).unwrap_or(0);

			let tasks = (remove_index..insert_index)
				.filter_map(|index| {
					<UnassignedTasks<T>>::get(network, index).and_then(|task_id| {
						Tasks::<T>::get(task_id)
							.filter(|task| {
								task.shard_size == shard_size
									&& (is_registered
										|| TaskPhaseState::<T>::get(task_id) != TaskPhase::Sign)
							})
							.map(|_| (index, task_id))
					})
				})
				.take(capacity)
				.collect::<Vec<_>>();
			for (index, task) in tasks {
				Self::assign_task(network, shard_id, index, task);
			}
		}

		fn assign_task(network: NetworkId, shard_id: ShardId, task_index: u64, task_id: TaskId) {
			if let Some(old_shard_id) = TaskShard::<T>::get(task_id) {
				ShardTasks::<T>::remove(old_shard_id, task_id);
			}
			Self::remove_unassigned_task(network, task_index);
			ShardTasks::<T>::insert(shard_id, task_id, ());
			TaskShard::<T>::insert(task_id, shard_id);
			Self::start_phase(shard_id, task_id, TaskPhaseState::<T>::get(task_id));
		}

		/// Apply the depreciation rate
		fn apply_depreciation(
			start: BlockNumberFor<T>,
			amount: BalanceOf<T>,
			rate: DepreciationRate<BlockNumberFor<T>>,
		) -> BalanceOf<T> {
			let now = frame_system::Pallet::<T>::block_number();
			let time_since_start = now.saturating_sub(start);
			if time_since_start.is_zero() {
				// no time elapsed since read phase started => full reward
				return amount;
			}
			let mut remaining = amount;
			let periods = time_since_start / rate.blocks;
			let mut i = BlockNumberFor::<T>::zero();
			while i < periods {
				remaining = remaining.saturating_sub(rate.percent * remaining);
				i = i.saturating_plus_one();
			}
			remaining
		}

		fn snapshot_write_reward(task_id: TaskId, signer: AccountId) {
			let Some(RewardConfig {
				write_task_reward,
				depreciation_rate,
				..
			}) = TaskRewardConfig::<T>::get(task_id)
			else {
				// reward config never stored => bug edge case
				return;
			};
			SignerPayout::<T>::insert(
				task_id,
				signer,
				Self::apply_depreciation(
					PhaseStart::<T>::take(task_id, TaskPhase::Write),
					write_task_reward,
					depreciation_rate,
				),
			);
		}

		fn payout_task_rewards(task_id: TaskId, shard_id: ShardId, phase: TaskPhase) {
			let task_account_id = Self::task_account(task_id);
			let start = PhaseStart::<T>::take(task_id, TaskPhase::Read);
			let Some(RewardConfig {
				read_task_reward,
				send_message_reward,
				depreciation_rate,
				..
			}) = TaskRewardConfig::<T>::take(task_id)
			else {
				return;
			};
			let mut shard_member_reward =
				Self::apply_depreciation(start, read_task_reward, depreciation_rate.clone());
			if phase == TaskPhase::Sign {
				let send_msg_reward =
					Self::apply_depreciation(start, send_message_reward, depreciation_rate);
				shard_member_reward = shard_member_reward.saturating_add(send_msg_reward);
			}
			// payout each member of the shard
			T::Shards::shard_members(shard_id).into_iter().for_each(|account| {
				let _ = pallet_balances::Pallet::<T>::transfer(
					&task_account_id,
					&account,
					shard_member_reward,
					ExistenceRequirement::AllowDeath,
				);
			});
			// payout write signer reward and cleanup storage
			SignerPayout::<T>::drain_prefix(task_id).for_each(|(account, amount)| {
				let _ = pallet_balances::Pallet::<T>::transfer(
					&task_account_id,
					&account,
					amount,
					ExistenceRequirement::AllowDeath,
				);
			});
		}

		fn get_gmp_hash(
			task_id: TaskId,
			shard_id: ShardId,
		) -> Result<Vec<u8>, sp_runtime::DispatchError> {
			let task_descriptor = Tasks::<T>::get(task_id).ok_or(Error::<T>::UnknownTask)?;
			let tss_public_key =
				T::Shards::tss_public_key(shard_id).ok_or(Error::<T>::UnknownShard)?;
			let network_id = T::Shards::shard_network(shard_id).ok_or(Error::<T>::UnknownShard)?;
			let gateway_contract =
				Gateway::<T>::get(network_id).ok_or(Error::<T>::GatewayNotRegistered)?;

			let gmp_params = GmpParams {
				network_id,
				tss_public_key,
				gateway_contract: gateway_contract.into(),
			};

			match task_descriptor.function {
				Function::SendMessage { msg } => {
					Ok(Message::gmp(msg).to_eip712_bytes(&gmp_params).into())
				},
				Function::RegisterShard { shard_id } => {
					let tss_public_key =
						T::Shards::tss_public_key(shard_id).ok_or(Error::<T>::UnknownShard)?;
					Ok(Message::update_keys([], [tss_public_key])
						.to_eip712_bytes(&gmp_params)
						.into())
				},
				Function::UnregisterShard { shard_id } => {
					let tss_public_key =
						T::Shards::tss_public_key(shard_id).ok_or(Error::<T>::UnknownShard)?;
					Ok(Message::update_keys([tss_public_key], [])
						.to_eip712_bytes(&gmp_params)
						.into())
				},
				_ => Err(Error::<T>::InvalidTaskFunction.into()),
			}
		}

		pub fn add_unassigned_task(network: NetworkId, task_id: TaskId) {
			let insert_index = UATasksInsertIndex::<T>::get(network).unwrap_or(0);
			UnassignedTasks::<T>::insert(network, insert_index, task_id);
			UATasksInsertIndex::<T>::insert(network, insert_index.saturating_add(1));
		}

		pub fn remove_unassigned_task(network: NetworkId, task_index: u64) {
			let insert_index = UATasksInsertIndex::<T>::get(network).unwrap_or(0);
			let mut remove_index = UATasksRemoveIndex::<T>::get(network).unwrap_or(0);

			if remove_index >= insert_index {
				return;
			}

			if task_index == remove_index {
				UnassignedTasks::<T>::remove(network, remove_index);
				remove_index = remove_index.saturating_add(1);
				while UnassignedTasks::<T>::get(network, remove_index).is_none()
					&& remove_index < insert_index
				{
					remove_index = remove_index.saturating_add(1);
				}
				UATasksRemoveIndex::<T>::insert(network, remove_index);
			} else {
				UnassignedTasks::<T>::remove(network, task_index);
			}
		}
	}

	impl<T: Config> TasksInterface for Pallet<T> {
		fn shard_online(shard_id: ShardId, network: NetworkId) {
			NetworkShards::<T>::insert(network, shard_id, ());
			if Gateway::<T>::get(network).is_some() {
				Self::register_shard(shard_id, network);
			}
			Self::schedule_tasks(network, Some(shard_id));
		}

		fn shard_offline(shard_id: ShardId, network: NetworkId) {
			NetworkShards::<T>::remove(network, shard_id);
			// unassign tasks
			ShardTasks::<T>::drain_prefix(shard_id).for_each(|(task_id, _)| {
				TaskShard::<T>::remove(task_id);
				Self::add_unassigned_task(network, task_id);
			});
			Self::unregister_shard(shard_id, network);
		}
	}
}
