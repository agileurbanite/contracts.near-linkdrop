use crate::utils::{CommonUtils, UserUtility};
use near_sdk::json_types::U128;
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_near_campaign_incorrect_name() {
  let initial_balance = to_yocto("100");
  let transfer_amount = to_yocto("50");
  let tokens_per_key = to_yocto("7");

  let (root, runtime) = CommonUtils::init_simulation();
  let user_utility = UserUtility::init(root, initial_balance);
  let contract = user_utility.contract;
  let pk = user_utility.public_key;

  let campaign_name = "new.campaign".to_string();

  let result = call!(
    contract.user_account,
    contract.create_near_campaign(
      campaign_name.clone(),
      pk.as_pk1(),
      7,
      U128::from(tokens_per_key),
      "testnet".parse().unwrap()
    ),
    deposit = transfer_amount
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  // One error should occur during the promise execute
  CommonUtils::assert_one_promise_error(result.clone(), "assertion failed");

  // The user's balance should not change
  let user_balance = CommonUtils::retrieve_account_balance(contract.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(user_balance, initial_balance);
}
