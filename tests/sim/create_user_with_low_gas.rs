use crate::utils::{assert_eq_with_gas, assert_one_promise_error, init_linkdrop, KeySet};
use near_sdk::serde_json::json;
use near_sdk_sim::to_yocto;

#[test]
fn create_user_with_low_gas() {
  let alice_initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("100");

  let (root, linkdrop, alice) = init_linkdrop(alice_initial_balance);
  let (public_key, _, _) = KeySet::create(0, 0).some_keys(0);
  let linkdrop_balance_start = linkdrop.account().unwrap().amount;

  let result = alice.call(
    linkdrop.account_id().clone(),
    "create_user_account",
    &json!({
      "name": alice.account_id.to_string(),
      "public_key": public_key
    })
      .to_string()
      .into_bytes(),
    1_000_000_000, // 1 GigaGas
    transfer_amount, // deposit
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  {
    let runtime = root.borrow_runtime();

    // One error must occur while running the method
    assert_one_promise_error(result.clone(), "Exceeded the prepaid gas");

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
