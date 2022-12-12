use crate as pallet_kitty;
use frame_support::traits::{ConstU32, ConstU64, Everything};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
	    Block = Block,
	    NodeBlock = Block,
	    UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Balance: pallet_balances,
		Timestamp: pallet_timestamp, 
        PalletKitty: pallet_kitty,


        // System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		// PalletKitty: pallet_kitty::{Pallet, Call, Storage, Event<T>},
		// Balance: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		// RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet},
		// Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type RuntimeEvent = RuntimeEvent; 
	type RuntimeCall = RuntimeCall;
	type RuntimeOrigin = RuntimeOrigin;
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl pallet_kitty::Config for Test {
    type RuntimeEvent = RuntimeEvent;
	type KittyRandomness = RandomnessCollectiveFlip;
	type Currency = Balance;
	type MaxOwnerKitty = ConstU32<3>;
	type KittyTime = Timestamp;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
