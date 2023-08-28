#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{ValueQuery, *};
	use frame_system::pallet_prelude::*;
	use sp_runtime::Saturating;
	use sp_std::vec::Vec;
	use time_primitives::{
		MemberEvents, Network, OcwShardInterface, PeerId, PublicKey, ShardId, ShardStatus,
		ShardsInterface, TasksInterface, TssPublicKey,
	};

	pub trait WeightInfo {
		fn register_shard() -> Weight;
	}

	impl WeightInfo for () {
		fn register_shard() -> Weight {
			Weight::default()
		}
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config<AccountId = sp_runtime::AccountId32> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type TaskScheduler: TasksInterface;
		#[pallet::constant]
		type MaxMembers: Get<u8>;
		#[pallet::constant]
		type MinMembers: Get<u8>;
		#[pallet::constant]
		type DkgTimeout: Get<BlockNumberFor<Self>>;
	}

	#[pallet::storage]
	pub type ShardCollectorPublicKey<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardId, PublicKey, OptionQuery>;

	#[pallet::storage]
	pub type ShardCollectorPeerId<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardId, PeerId, OptionQuery>;

	#[pallet::storage]
	/// Counter for creating unique shard_ids during on-chain creation
	pub type ShardIdCounter<T: Config> = StorageValue<_, ShardId, ValueQuery>;

	/// Network for which shards can be assigned tasks
	#[pallet::storage]
	pub type ShardNetwork<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardId, Network, OptionQuery>;

	/// Status for shard
	#[pallet::storage]
	pub type ShardState<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardId, ShardStatus<BlockNumberFor<T>>, OptionQuery>;

	/// Threshold for shard
	#[pallet::storage]
	pub type ShardThreshold<T: Config> = StorageMap<_, Blake2_128Concat, ShardId, u16, OptionQuery>;

	#[pallet::storage]
	pub type ShardPublicKey<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardId, TssPublicKey, OptionQuery>;

	#[pallet::storage]
	pub type MemberShard<T: Config> = StorageMap<_, Blake2_128Concat, PeerId, ShardId, OptionQuery>;

	#[pallet::storage]
	pub type ShardMembers<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, ShardId, Blake2_128Concat, PeerId, (), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New shard was created
		ShardCreated(ShardId, Network),
		/// Shard DKG timed out
		ShardKeyGenTimedOut(ShardId),
		/// Shard completed dkg and submitted public key to runtime
		ShardOnline(ShardId, TssPublicKey),
		/// Shard went offline
		ShardOffline(ShardId),
	}

	#[pallet::error]
	pub enum Error<T> {
		UnknownShard,
		PublicKeyAlreadyRegistered,
		MembershipBelowMinimum,
		MembershipAboveMaximum,
		ThresholdAboveMembershipLen,
		ThresholdMustBeNonZero,
		ShardAlreadyOffline,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Root can register new shard via providing
		/// set of IDs matching one of supported size of shard
		/// # Param
		/// * members - supported sized set of shard members Id
		/// * collector - index of collector if not index 0
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::register_shard())]
		pub fn register_shard(
			origin: OriginFor<T>,
			network: Network,
			members: Vec<PeerId>,
			collector: PublicKey,
			threshold: u16,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(
				members.len() >= T::MinMembers::get().into(),
				Error::<T>::MembershipBelowMinimum
			);
			ensure!(
				members.len() <= T::MaxMembers::get().into(),
				Error::<T>::MembershipAboveMaximum
			);
			ensure!(members.len() >= threshold as usize, Error::<T>::ThresholdAboveMembershipLen);
			ensure!(threshold > 0, Error::<T>::ThresholdMustBeNonZero);
			Self::create_shard(network, members, collector, threshold);
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let mut writes = 0;
			ShardState::<T>::iter().for_each(|(shard_id, status)| {
				if let Some(created_block) = status.when_created() {
					if n.saturating_sub(created_block) >= T::DkgTimeout::get() {
						// clean up shard state and emit event for timeout
						ShardState::<T>::remove(shard_id);
						ShardNetwork::<T>::remove(shard_id);
						let _ = ShardMembers::<T>::clear_prefix(shard_id, u32::max_value(), None);
						ShardCollectorPublicKey::<T>::remove(shard_id);
						Self::deposit_event(Event::ShardKeyGenTimedOut(shard_id));
						writes += 5;
					}
				}
			});
			T::DbWeight::get().writes(writes)
		}
	}

	impl<T: Config> Pallet<T> {
		fn create_shard(
			network: Network,
			members: Vec<PeerId>,
			collector: PublicKey,
			threshold: u16,
		) {
			let shard_id = <ShardIdCounter<T>>::get();
			<ShardIdCounter<T>>::put(shard_id + 1);
			<ShardNetwork<T>>::insert(shard_id, network);
			<ShardState<T>>::insert(
				shard_id,
				ShardStatus::Created(frame_system::Pallet::<T>::block_number()),
			);
			<ShardThreshold<T>>::insert(shard_id, threshold);
			<ShardCollectorPublicKey<T>>::insert(shard_id, collector);
			//TODO change on dynamic collector assigning
			<ShardCollectorPeerId<T>>::insert(shard_id, members[0]);
			for member in &members {
				<ShardMembers<T>>::insert(shard_id, *member, ());
				MemberShard::<T>::insert(*member, shard_id);
			}
			Self::deposit_event(Event::ShardCreated(shard_id, network));
		}

		pub fn get_shard_threshold(shard_id: ShardId) -> u16 {
			ShardThreshold::<T>::get(shard_id).unwrap_or_default()
		}

		pub fn get_shards(peer_id: PeerId) -> Vec<ShardId> {
			ShardMembers::<T>::iter()
				.filter_map(
					|(shard_id, member, _)| {
						if member == peer_id {
							Some(shard_id)
						} else {
							None
						}
					},
				)
				.collect()
		}

		pub fn get_shard_members(shard_id: ShardId) -> Vec<PeerId> {
			ShardMembers::<T>::iter_prefix(shard_id).map(|(time_id, _)| time_id).collect()
		}

		// Used in tests
		pub fn set_shard_offline(shard_id: ShardId) -> DispatchResult {
			let shard_state = ShardState::<T>::get(shard_id).ok_or(Error::<T>::UnknownShard)?;
			let network = ShardNetwork::<T>::get(shard_id).ok_or(Error::<T>::UnknownShard)?;
			ensure!(!matches!(shard_state, ShardStatus::Offline), Error::<T>::ShardAlreadyOffline);
			<ShardState<T>>::insert(shard_id, ShardStatus::Offline);
			Self::deposit_event(Event::ShardOffline(shard_id));
			T::TaskScheduler::shard_offline(shard_id, network);
			Ok(())
		}
	}

	impl<T: Config> MemberEvents for Pallet<T> {
		fn member_online(id: &PeerId) {
			let shard_id = MemberShard::<T>::get(id).unwrap();
			let old_status = ShardState::<T>::get(shard_id).unwrap();
			let new_status = old_status.online_member();
			ShardState::<T>::insert(shard_id, new_status);
		}
		fn member_offline(id: &PeerId) {
			let shard_id = MemberShard::<T>::get(id).unwrap();
			let old_status = ShardState::<T>::get(shard_id).unwrap();
			let shard_threshold = ShardThreshold::<T>::get(shard_id).unwrap();
			let total_members = ShardMembers::<T>::iter().collect::<Vec<_>>().len();
			let max_members_offline = total_members.saturating_sub(shard_threshold.into());
			let new_status = old_status.offline_member(max_members_offline.try_into().unwrap());
			ShardState::<T>::insert(shard_id, new_status);
			if matches!(new_status, ShardStatus::Offline) {
				let network = ShardNetwork::<T>::get(shard_id).unwrap();
				Self::deposit_event(Event::ShardOffline(shard_id));
				T::TaskScheduler::shard_offline(shard_id, network);
			}
		}
	}

	impl<T: Config> OcwShardInterface for Pallet<T> {
		fn benchmark_register_shard(
			network: Network,
			members: Vec<PeerId>,
			collector: PublicKey,
			threshold: u16,
		) {
			Self::create_shard(network, members, collector, threshold);
		}
		fn submit_tss_public_key(shard_id: ShardId, public_key: TssPublicKey) -> DispatchResult {
			let network = ShardNetwork::<T>::get(shard_id).ok_or(Error::<T>::UnknownShard)?;
			ensure!(
				ShardPublicKey::<T>::get(shard_id).is_none(),
				Error::<T>::PublicKeyAlreadyRegistered
			);
			<ShardPublicKey<T>>::insert(shard_id, public_key);
			<ShardState<T>>::insert(shard_id, ShardStatus::Online);
			Self::deposit_event(Event::ShardOnline(shard_id, public_key));
			T::TaskScheduler::shard_online(shard_id, network);
			Ok(())
		}
	}

	impl<T: Config> ShardsInterface for Pallet<T> {
		fn is_shard_online(shard_id: ShardId) -> bool {
			matches!(ShardState::<T>::get(shard_id), Some(ShardStatus::Online))
		}

		fn collector_pubkey(shard_id: ShardId) -> Option<PublicKey> {
			ShardCollectorPublicKey::<T>::get(shard_id)
		}

		fn collector_peer_id(shard_id: ShardId) -> Option<PeerId> {
			ShardCollectorPeerId::<T>::get(shard_id)
		}
	}
}
