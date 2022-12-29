#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{
		*, ValueQuery, DispatchResult}, 
		traits::{Randomness, Currency, Time, fungibles::{Create, Destroy, Transfer, metadata::Mutate as MetadataMute}, tokens::AssetId}, 
		ensure, transactional, BoundedVec};
	use frame_system::{pallet_prelude::OriginFor, ensure_signed};
	use sp_runtime::traits::CheckedAdd;

	use frame_support::inherent::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ From<u32>;

		type AssetHandler: Create<Self::AccountId, AssetId = Self::AssetId>
			+ Destroy<Self::AccountId>
			+ Transfer<Self::AccountId>
			+ MetadataMute<Self::AccountId>;

	}

	#[pallet::storage]
	#[pallet::getter(fn asset_count)]
	pub type AssetCount<T> = StorageValue<_, u32, ValueQuery, >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetLimited,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_token(origin: OriginFor<T>, asset_name: Vec<u8>, asset_symbol: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			let asset_id: T::AssetId = Self::asset_count().into();

			// create new token
			T::AssetHandler::create(
				asset_id,
				who.clone(),
				true,
				1_u32.into()
			)?;

			// set metadata for token
			T::AssetHandler::set(
				asset_id,
				&who,
				asset_name,
				asset_symbol,
				10
			)?;

			let next_asset_id = Self::asset_count().checked_add(1_u32).ok_or(<Error<T>>::AssetLimited)?;
			
			<AssetCount<T>>::put(next_asset_id);

			Ok(())
		}

	}
}
