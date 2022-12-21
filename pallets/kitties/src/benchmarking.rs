//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PalletKitty;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(PalletKitty, crate::mock::new_test_ext(), crate::mock::Test);
}