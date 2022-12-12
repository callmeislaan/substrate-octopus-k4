#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{
		*, ValueQuery, DispatchResult}, 
		traits::{Randomness, Currency, Time}, 
		ensure, transactional};
	use frame_support::sp_runtime::traits::Hash;
	use frame_system::{pallet_prelude::OriginFor, ensure_signed};
	use sp_io::hashing::blake2_128;

	use frame_support::log;

	use crate::types::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub(crate) type AccountOf<T> = <T as frame_system::Config>::AccountId;

	pub(crate) type TimeOf<T> = <<T as Config>::KittyTime as frame_support::traits::Time>::Moment;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type KittyRandomness: Randomness<<Self as frame_system::Config>::Hash, <Self as frame_system::Config>::BlockNumber>;

		type Currency: Currency<Self::AccountId>;

		type MaxOwnerKitty: Get<u32>;

		type KittyTime: Time;

	}

	#[pallet::storage]
	#[pallet::getter(fn count)]
	pub type KittyCounter<T> = StorageValue<_, u32, ValueQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitty<T>, OptionQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::Hash, T::MaxOwnerKitty>, ValueQuery, >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(AccountOf<T>, T::Hash),
		KittyTranfered(T::Hash, AccountOf<T>),
		PriceSet(T::Hash, Option<BalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		KittyNotExists,
		NotKittyOwner,
		TransferToSelf,
		KittyHasNoPrice,
		MaxOwnerKitty,
		KittyPriceNone,
		NotEnoughMoney,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty = Self::mint(who.clone());


			let current_kitty_count = Self::count();

			<KittyCounter<T>>::put(current_kitty_count + 1);

			let kitty_id = T::Hashing::hash_of(&kitty);

			<Kitties<T>>::insert(kitty_id.clone(), kitty.clone());

			let mut kitty_owner = Self::kitty_owner(&who);

			kitty_owner.try_push(kitty_id.clone()).map_err(|_| <Error<T>>::MaxOwnerKitty)?;

			<KittyOwner<T>>::insert(who.clone(), kitty_owner.clone());
		

			log::info!("Create new Kitty: {:?}", kitty);

			Self::deposit_event(Event::KittyCreated(who, kitty_id));

			Ok(())
		}
		
		#[pallet::weight(10_000)]
		pub fn set_price(origin: OriginFor<T>, kitty_id: T::Hash, price: Option<BalanceOf<T>>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			ensure!(Self::is_kitty_owner(&kitty_id, &who)?, <Error<T>>::NotKittyOwner);

			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExists)?;

			kitty.set_price(price);

			<Kitties<T>>::insert(kitty_id.clone(), kitty);

			Self::deposit_event(<Event<T>>::PriceSet(kitty_id, price));


			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: T::Hash) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// ensure the kitty owner
			ensure!(Self::is_kitty_owner(&kitty_id, &from)?, <Error<T>>::NotKittyOwner);

			// ensure not transfer to self
			ensure!(from != to, <Error<T>>::TransferToSelf);

			Self::transfer_to(kitty_id, &to)?;

			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::Hash) -> DispatchResult {

			let buyer = ensure_signed(origin)?;

			let kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExists)?;

			let kitty_price = kitty.price().ok_or(<Error<T>>::KittyPriceNone)?;

			// ensure buyer has enough money
			ensure!(T::Currency::free_balance(&buyer) >= kitty_price, <Error<T>>::NotEnoughMoney);

			let owner = kitty.owner();

			// ensure not transfer to self
			ensure!(owner != buyer, <Error<T>>::TransferToSelf);

			Self::transfer_to(kitty_id, &buyer)?;

			Ok(())
		}

	}

	
	/// helper 
	impl <T:Config> Pallet<T> {

		fn generate_dna() -> [u8; 16] {

			let payload = (
				T::KittyRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::extrinsic_index().unwrap_or_default(),
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		fn generate_gender(dna: &[u8; 16] ) -> Gender {
			if dna[0] % 2 == 0 {
				return Gender::MALE;
			}
			Gender::FEMALE
		}
	
		fn mint(who: T::AccountId) -> Kitty<T> {
			let dna = Self::generate_dna();
			let gender = Self::generate_gender(&dna);
			let now = T::KittyTime::now();
			<Kitty<T>>::new(who, dna, gender, now)
		}

		fn is_kitty_owner(kitty_id: &T::Hash, owner: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty.owner() == *owner),
				None => Err(Error::KittyNotExists),
			}
		}

		fn transfer_to(kitty_id: T::Hash, to: &T::AccountId) -> Result<(), Error<T>> { 

			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExists)?;

			let owner = kitty.owner();

			// remove old kitty owner
			<KittyOwner<T>>::try_mutate(&owner, |kitty_vec| {
				if let Some(position) = kitty_vec.iter().position(|x| *x == kitty_id.clone()) {
					kitty_vec.swap_remove(position);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::KittyNotExists)?;

			// update kitty owner

			kitty.set_price(None);
			kitty.set_owner(to.clone());
			<Kitties<T>>::insert(kitty_id.clone(), kitty);

			// add new kitty owner s
			<KittyOwner<T>>::try_mutate(&to, |kitty_vec| {
				kitty_vec.try_push(kitty_id)
			}).map_err(|_| <Error<T>>::KittyNotExists)?;

			Ok(())

		}
	}

}
