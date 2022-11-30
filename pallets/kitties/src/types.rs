#![cfg_attr(not(feature = "std"), no_std)]

use core::{clone, fmt::{Display, write}};

use codec::*;
use frame_support::{inherent::Vec, RuntimeDebug};
use scale_info::TypeInfo;

use crate::Config;
use crate::AccountOf;
use crate::BalanceOf;


#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
    dna: Vec<u8>,
    owner: AccountOf<T>,
    price: Option<BalanceOf<T>>,
    gender: Gender,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Gender {
    MALE,
    FEMALE
}

impl <T: Config> Kitty<T> {

    pub fn new(who: T::AccountId, dna: Vec<u8>, gender: Gender) -> Self {
        Kitty {
            dna,
            owner: who,
            price: None,
            gender,
        }
    }

    pub fn dna(&self) -> Vec<u8> {
        self.dna.clone()
    }

    pub fn owner(&self) -> T::AccountId {
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

    pub fn set_owner(&mut self, new_owner: T::AccountId) {
        self.owner = new_owner
    }
}

impl <T> sp_std::fmt::Display for Kitty<T> where T: Config {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "(dna: {:?}, price: {:?}, gender: {:?}, owner: {:?}", self.dna, self.price, self.gender, self.owner)
    }
}
