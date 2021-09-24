use crate::utils::{init_linkdrop, KeySet};
use near_sdk_sim::{call, to_yocto};

#[test]
#[should_panic(expected = r#"NotEnoughBalance"#)]
fn create_user_not_enough_balance() {
  let alice_initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("200");

  let (_root, linkdrop, alice) = init_linkdrop(alice_initial_balance);
  let (public_key, _, _) = KeySet::create(0, 0).some_keys(0);

  call!(
    alice,
    linkdrop.create_user_account(alice.account_id.to_string(), public_key),
    deposit = transfer_amount
  );
}
