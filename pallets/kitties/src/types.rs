#![cfg_attr(not(feature = "std"), no_std)]

use codec::*;
use frame_support::{RuntimeDebug};
use scale_info::TypeInfo;

use crate::Config;
use crate::AccountOf;
use crate::BalanceOf;
use crate::TimeOf;


#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
    dna: [u8; 16],
    owner: AccountOf<T>,
    price: Option<BalanceOf<T>>,
    gender: Gender,
    created_date: TimeOf<T>,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
    MALE,
    FEMALE
}

impl <T: Config> Kitty<T> {

    pub fn new(who: AccountOf<T>, dna: [u8; 16], gender: Gender, created_date: TimeOf<T>) -> Self {
        Kitty {
            dna,
            owner: who,
            price: None,
            gender,
            created_date,
        }
    }

    pub fn dna(&self) -> [u8; 16] {
        self.dna.clone()
    }

    pub fn owner(&self) -> AccountOf<T> {
        self.owner.clone()
    }

    pub fn price(&self) -> Option<BalanceOf<T>> {
        self.price
    }

    pub fn gender(&self) -> Gender {
        self.gender.clone()
    }

    pub fn set_price(&mut self, new_price: Option<BalanceOf<T>>) {
        self.price = new_price;
    }

    pub fn set_owner(&mut self, new_owner: AccountOf<T>) {
        self.owner = new_owner
    }

    pub fn created_date(&self) -> TimeOf<T> {
        self.created_date
    }
}

impl <T> sp_std::fmt::Display for Kitty<T> where T: Config {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "(dna: {:?}, price: {:?}, gender: {:?}, owner: {:?}, created_date: {:?}", self.dna, self.price, self.gender, self.owner, self.created_date)
    }
}
