#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;

pub mod weights;
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::{types::*, weights::WeightInfo};

	use onchain_task::types as task_types;

	type BlockHeight = u64;

	use frame_support::{pallet_prelude::*, sp_runtime::traits::Scale, traits::Time};
	use frame_system::pallet_prelude::*;
	use scale_info::StaticTypeInfo;
	use time_primitives::{SignatureData, TimeId, TimeSignature};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type Moment: Parameter
			+ Default
			+ Scale<Self::BlockNumber, Output = Self::Moment>
			+ Copy
			+ MaxEncodedLen
			+ StaticTypeInfo;
		type Timestamp: Time<Moment = Self::Moment>;
	}

	#[pallet::storage]
	#[pallet::getter(fn tesseract_members)]
	pub type TesseractMembers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, TesseractRole, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn signature_store)]
	pub type SignatureStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, SignatureData, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn signature_storage)]
	pub type SignatureStoreData<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		task_types::TaskId,
		Blake2_128Concat,
		BlockHeight,
		SignatureStorage<T::Moment>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The event data for stored signature
		/// the signature id that uniquely identify the signature
		SignatureStored(SignatureData),

		/// A tesseract Node has been added as a member with it's role
		TesseractMemberAdded(T::AccountId, TesseractRole),

		/// A tesseract Node has been removed
		TesseractMemberRemoved(T::AccountId),

		/// Unauthorized attempt to add signed data
		UnregisteredWorkerDataSubmission(T::AccountId),

		/// Default account is not allowed for this operation
		DefaultAccountForbidden(),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The Tesseract address in not known
		UnknownTesseract,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Extrinsic for storing a signature
		#[pallet::weight(T::WeightInfo::store_signature_data())]
		pub fn store_signature(
			origin: OriginFor<T>,
			signature_data: SignatureData,
			task_id: task_types::TaskId,
			block_height: u64,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			ensure!(TesseractMembers::<T>::contains_key(caller), Error::<T>::UnknownTesseract);
			let storage_data = SignatureStorage::new(signature_data.clone(), T::Timestamp::now());

			<SignatureStoreData<T>>::insert(task_id, block_height, storage_data);

			Self::deposit_event(Event::SignatureStored(signature_data));

			Ok(())
		}

		/// Extrinsic for adding a node and it's member role
		/// Callable only by root for now
		#[pallet::weight(T::WeightInfo::add_member())]
		pub fn add_member(
			origin: OriginFor<T>,
			account: T::AccountId,
			role: TesseractRole,
		) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;

			<TesseractMembers<T>>::insert(account.clone(), role.clone());

			Self::deposit_event(Event::TesseractMemberAdded(account, role));

			Ok(())
		}

		/// Extrinsic for adding a node and it's member role
		/// Callable only by root for now
		#[pallet::weight(T::WeightInfo::remove_member())]
		pub fn remove_member(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;

			<TesseractMembers<T>>::remove(account.clone());

			Self::deposit_event(Event::TesseractMemberRemoved(account));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn api_store_signature(
			auth_id: TimeId,
			auth_sig: TimeSignature,
			signature_data: SignatureData,
			task_id: task_types::TaskId,
			block_height: u64,
		) {
			use sp_runtime::traits::Verify;
			// transform AccountId32 int T::AccountId
			let encoded_account = auth_id.encode();
			if encoded_account.len() != 32 || encoded_account == [0u8; 32].to_vec() {
				Self::deposit_event(Event::DefaultAccountForbidden());
				return;
			}
			// Unwrapping is safe - we've checked for len and default-ness
			let account_id = T::AccountId::decode(&mut &*encoded_account).unwrap();
			if !TesseractMembers::<T>::contains_key(account_id.clone())
				|| !auth_sig.verify(&*signature_data, &auth_id)
			{
				Self::deposit_event(Event::UnregisteredWorkerDataSubmission(account_id));
				return;
			}
			let storage_data = SignatureStorage::new(signature_data.clone(), T::Timestamp::now());

			<SignatureStoreData<T>>::insert(task_id, block_height, storage_data);

			Self::deposit_event(Event::SignatureStored(signature_data));
		}
	}
}
