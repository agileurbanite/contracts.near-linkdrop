use crate::utils::{assert_eq_with_gas, init_linkdrop, KeySet};
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_user() {
  let initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("100");

  let (root, linkdrop, alice) = init_linkdrop(initial_balance);
  let key_set = KeySet::create(0, 0);
  let (public_key, pub_key, _) = key_set.some_keys(0);
  let new_account_id = "alice.linkdrop";

  let result = call!(
    alice,
    linkdrop.create_user_account(alice.account_id.to_string(), public_key),
    deposit = transfer_amount
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // Check Alice balance
    let alice_balance = runtime
      .view_account(alice.account_id.as_str())
      .unwrap()
      .amount;
    assert_eq_with_gas(to_yocto("100"), alice_balance);

    // Check Alice Linkdrop balance
    let alice_linkdrop_balance = runtime.view_account(new_account_id).unwrap().amount;
    assert_eq_with_gas(transfer_amount, alice_linkdrop_balance);

    // Check Alice access key
    let key = runtime.view_access_key(new_account_id, &pub_key);
    assert_eq!(key.is_some(), true);
  }
}
