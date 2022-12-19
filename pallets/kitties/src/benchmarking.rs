// #![cfg(feature = "runtime-benchmarks")]

use crate::mock::*;

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	impl_benchmark_test_suite!(PalletKitty, new_test_ext(), crate::mock::Test);
}