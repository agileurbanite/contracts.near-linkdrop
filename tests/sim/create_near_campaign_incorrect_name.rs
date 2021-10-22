use crate::utils::{assert_eq_with_gas, assert_one_promise_error, init_user_contract, KeySet};
use near_sdk::json_types::U128;
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_near_campaign_incorrect_name() {
  let initial_balance = to_yocto("100");
  let transfer_amount = to_yocto("50");
  let tokens_per_key = to_yocto("7");

  let (root, user_contract) = init_user_contract(initial_balance);
  let key_set = KeySet::create(0, 0);
  let (public_key, _, _) = key_set.some_keys(0);

  let campaign_name = "new.campaign".to_string();

  let result = call!(
    user_contract.user_account,
    user_contract.create_near_campaign(
      campaign_name.clone(),
      public_key,
      7,
      U128::from(tokens_per_key),
      user_contract.account_id()
      ),
    deposit = transfer_amount
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  {
    let runtime = root.borrow_runtime();

    // One error should occur during the promise execute
    assert_one_promise_error(result.clone(), "assertion failed");

    // The user's balance should not change
    let user_balance = runtime
      .view_account(user_contract.account_id().as_str())
      .unwrap()
      .amount;
    assert_eq_with_gas(user_balance, initial_balance);
  }
}
