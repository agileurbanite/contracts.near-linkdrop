use crate::utils::{init_user_contract, KeySet, assert_eq_with_gas};
use near_sdk::json_types::U128;
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_near_campaign() {
  let initial_balance = to_yocto("100");
  let transfer_amount = to_yocto("50");
  let tokens_per_key = to_yocto("7");

  let (root, user_contract) = init_user_contract(initial_balance);
  let key_set = KeySet::create(0, 0);
  let (public_key, pub_key, _) = key_set.some_keys(0);

  let campaign_name = "new_campaign".to_string();
  let campaign_account_id = format!(
    "{}.{}",
    campaign_name,
    user_contract.account_id()
  );

  let result = call!(
    user_contract.user_account,
    user_contract.create_near_campaign(
      campaign_name,
      public_key,
      U128::from(tokens_per_key),
      user_contract.account_id()
    ),
    deposit = transfer_amount
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // Check User balance
    let user_balance = runtime.view_account(
      user_contract.account_id().as_str()
    ).unwrap().amount;
    assert_eq_with_gas(
      to_yocto("50"),
      user_balance
    );

    // Check Campaign balance
    let campaign_balance = runtime.view_account(
      campaign_account_id.as_str()
    ).unwrap().amount;
    assert_eq_with_gas(
      transfer_amount,
      campaign_balance
    );

    // Check New Campaign access key
    let key = runtime.view_access_key(
      campaign_account_id.as_str(),
      &pub_key
    );
    assert_eq!(key.is_some(), true);
  }
}
