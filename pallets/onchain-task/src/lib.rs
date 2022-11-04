#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;

pub mod weights;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use crate::weights::WeightInfo;


	use crate::types::*;
	
	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn tesseract_tasks)]
	pub type TesseractTasks<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, TesseractTask, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task_store)]
	pub type OnchainTaskStore<T: Config> =
		StorageMap<_, Blake2_128Concat, ChainKey, ChainData, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The chain id that uniquely identify the chain data
		OnchainDataStored(ChainKey),

		/// A tesseract task has been added
		TesseractTaskAdded(T::AccountId, TesseractTask),

		/// A tesseract task removed
		TesseractTaskRemoved(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The Tesseract task is not known
		UnknownTask,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Extrinsic for storing onchain data
		#[pallet::weight(
			T::WeightInfo::store_onchain_task()
		)]
		pub fn store_onchain_task(
			origin: OriginFor<T>,
			chain_key: ChainKey,
			chain_data: ChainData,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			ensure!(TesseractTasks::<T>::contains_key(caller), Error::<T>::UnknownTask);

			<OnchainTaskStore<T>>::insert(
				chain_key.clone(), chain_data.clone(),
			);

			Self::deposit_event(Event::OnchainDataStored(chain_key));

			Ok(())
		}

		/// Extrinsic for adding a node's task
		/// Callable only by root for now
		#[pallet::weight(T::WeightInfo::add_task())]
		pub fn add_task(
			origin: OriginFor<T>,
			account: T::AccountId,
			task: TesseractTask,
		) -> DispatchResult {
			_ = ensure_root(origin)?;
			<TesseractTasks<T>>::insert(account.clone(), task.clone());

			Self::deposit_event(Event::TesseractTaskAdded(account, task));

			Ok(())
		}

		/// Extrinsic for adding a node's task
		/// Callable only by root for now
		#[pallet::weight(T::WeightInfo::remove_task())]
		pub fn remove_task(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			_ = ensure_root(origin)?;

			<TesseractTasks<T>>::remove(account.clone());

			Self::deposit_event(Event::TesseractTaskRemoved(account));

			Ok(())
		}
	}
}
