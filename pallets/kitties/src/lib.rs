#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, ValueQuery, DispatchResult}, traits::{Randomness, Currency}, ensure};
	use frame_system::{pallet_prelude::{*, OriginFor}, ensure_signed};
	use frame_support::inherent::Vec;
	use sp_io::hashing::blake2_128;

	use crate::types::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	pub(super) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub(super) type AccountOf<T> = <T as frame_system::Config>::AccountId;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type KittyRandomness: Randomness<<Self as frame_system::Config>::Hash, <Self as frame_system::Config>::BlockNumber>;

		type Currency: Currency<Self::AccountId>;

	}

	#[pallet::storage]
	#[pallet::getter(fn count)]
	pub type KittyCounter<T> = StorageValue<_, u32, ValueQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery, >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(AccountOf<T>, Vec<u8>),
		KittyTranfered(Vec<u32>, AccountOf<T>),
		PriceSet(Vec<u8>, Option<BalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		KittyNotExist,
		NotKittyOwner,
		TransferToSelf,
		KittyHasNoPrice,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty = Self::mint(who.clone());


			let current_kitty_count = Self::count();

			<KittyCounter<T>>::put(current_kitty_count + 1);

			<Kitties<T>>::insert(kitty.dna(), kitty.clone());


			let mut kitty_owner = Self::kitty_owner(&who);

			kitty_owner.push(kitty.dna());

			<KittyOwner<T>>::insert(who.clone(), kitty_owner.clone());
			

			Self::deposit_event(Event::KittyCreated(who, kitty.dna()));

			Ok(())
		}
		
		#[pallet::weight(10_000)]
		pub fn set_price(origin: OriginFor<T>, dna: Vec<u8>, price: Option<BalanceOf<T>>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			ensure!(Self::is_kitty_owner(&dna, &who)?, <Error<T>>::NotKittyOwner);

			let mut kitty = Self::kitties(&dna).ok_or(<Error<T>>::NotKittyOwner)?;

			kitty.set_price(price);

			<Kitties<T>>::insert(dna.clone(), kitty);

			Self::deposit_event(<Event<T>>::PriceSet(dna, price));


			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: Vec<u8>) -> DispatchResult {
			let from = ensure_signed(origin)?;

			Self::transfer_to(dna, &from, &to)?;

			Ok(())
		}

	}

	
	/// helper 
	impl <T:Config> Pallet<T> {

		fn generate_dna() -> Vec<u8> {

			let payload = (
				T::KittyRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::extrinsic_index().unwrap_or_default(),
				<frame_system::Pallet<T>>::block_number(),
			);
			let encoded = payload.using_encoded(blake2_128);
			encoded.to_vec()
		}

		fn generate_gender(dna: &Vec<u8> ) -> Gender {
			if dna[0] % 2 == 0 {
				return Gender::MALE;
			}
			Gender::FEMALE
		}
	
		fn mint(who: T::AccountId) -> Kitty<T> {
			let dna = Self::generate_dna();
			let gender = Self::generate_gender(&dna);
			<Kitty<T>>::new(who, dna, gender)
		}

		fn is_kitty_owner(dna: &Vec<u8>, owner: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::kitties(dna) {
				Some(kitty) => Ok(kitty.owner() == *owner),
				None => Err(Error::KittyNotExist),
			}
		}

		fn transfer_to(dna: Vec<u8>, from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> { 
			// ensure the kitty owner
			ensure!(Self::is_kitty_owner(&dna, from)?, <Error<T>>::NotKittyOwner);

			// ensure not transfer to self
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// remove old kitty owner
			<KittyOwner<T>>::try_mutate(from, |kitty_vec| {
				if let Some(position) = kitty_vec.iter().position(|x| *x == dna.clone()) {
					kitty_vec.swap_remove(position);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::KittyNotExist)?;

			// update kitty owner

			let mut kitty = Self::kitties(&dna).ok_or(<Error<T>>::KittyNotExist)?;
			kitty.set_price(None);
			kitty.set_owner(to.clone());
			<Kitties<T>>::insert(dna.clone(), kitty);

			// add new kitty owner s
			<KittyOwner<T>>::try_mutate(&to, |kitty_vec| {
				kitty_vec.push(dna);
				Ok(())
			}).map_err(|_: Result<(), Error<T>>| <Error<T>>::KittyNotExist)?;

			Ok(())

		}
	}

}
