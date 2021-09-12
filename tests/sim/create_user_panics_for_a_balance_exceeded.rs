use crate::utils::{init_linkdrop, KeySet};
use near_sdk_sim::{call, to_yocto};

#[test]
#[should_panic(expected = r#"NotEnoughBalance"#)]
fn create_user_panics_for_a_balance_exceeded() {
  let initial_balance = to_yocto("100");
  let transfer_amount = to_yocto("200");
  let (_root, linkdrop, alice) = init_linkdrop(initial_balance);
  let (public_key, _, _) = KeySet::create(0, 0).some_keys(0);

  call!(
    alice,
    linkdrop.create_user_account(alice.account_id.to_string(), public_key),
    deposit = transfer_amount
  );
}
