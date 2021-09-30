use crate::utils::{assert_eq_with_gas, assert_one_promise_error, init_linkdrop, KeySet};
use near_sdk::AccountId;
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_user_name_is_already_exists() {
  let alice_initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("100");

  let (root, linkdrop, alice) = init_linkdrop(alice_initial_balance);
  let (public_key, _, _) = KeySet::create(0, 0).some_keys(0);

  let user_account_id = format!("{}.{}", alice.account_id(), linkdrop.account_id());
  linkdrop.user_account.create_user(
    AccountId::new_unchecked(user_account_id),
    to_yocto("10"),
  );
  let linkdrop_balance_start = linkdrop.account().unwrap().amount;

  let result = call!(
    alice,
    linkdrop.create_user_account(alice.account_id.to_string(), public_key),
    deposit = transfer_amount
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // One error must occur while running the method
    assert_one_promise_error(
      result.clone(),
      "Can't create a new account \"alice.linkdrop\", because it already exists"
    );

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("Is user created: false"));

    // Alice's balance has not changed
    let alice_balance = runtime
      .view_account(alice.account_id.as_str())
      .unwrap()
      .amount;
    assert_eq_with_gas(alice_initial_balance, alice_balance);

    // The balance of the contract has not changed
    let linkdrop_balance_end = runtime
      .view_account(linkdrop.account_id().as_str())
      .unwrap()
      .amount;
    assert_eq_with_gas(linkdrop_balance_start, linkdrop_balance_end);
  }
}
