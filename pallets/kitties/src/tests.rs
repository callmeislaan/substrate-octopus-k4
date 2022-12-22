use crate::mock::*;

use frame_support::{assert_ok};

#[test]
fn create_new_kitty_should_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let sender = 1;

        assert_ok!(PalletKitty::create_kitty(RuntimeOrigin::signed(sender)));
    })
}

#[test]
fn transfer_kitty_should_work() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let owner = 1;
        let to = 2;
        let owner_origin = RuntimeOrigin::signed(owner); 
        assert_ok!(PalletKitty::create_kitty(owner_origin.clone()));
        
        let kitty_id = PalletKitty::kitty_owner(owner).get(0).unwrap().clone();

        assert_ok!(PalletKitty::transfer(owner_origin, to, kitty_id));

    })
}