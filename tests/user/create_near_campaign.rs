use crate::utils::{CommonUtils, UserUtility};
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use near_sdk_sim::{call, view, to_yocto};

#[test]
fn create_near_campaign() {
  let initial_balance = to_yocto("100");
  let transfer_amount = to_yocto("50");
  let tokens_per_key = to_yocto("7");

  let (root, runtime) = CommonUtils::init_simulation();
  let user_utility = UserUtility::init(root, initial_balance);
  let contract = user_utility.contract;
  let pk = user_utility.public_key;

  let campaign_name = "new_campaign".to_string();

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
  result.assert_success();

  // Check User balance
  let user_balance = CommonUtils::retrieve_account_balance(contract.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(
    to_yocto("50"), // 100 - 50 NEAR
    user_balance
  );

  // Check Campaign balance
  let campaign_account_id = format!("{}.{}", campaign_name, contract.account_id());
  let campaign_balance = CommonUtils::retrieve_account_balance(campaign_account_id.as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(transfer_amount, campaign_balance);

  // Check New Campaign access key
  let key = CommonUtils::retrieve_account_access_key(campaign_account_id.as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("Is campaign created: true"));

  // Check the result of the callback function
  let campaigns: Vec<AccountId> = view!(contract.get_campaigns()).unwrap_json();
  assert_eq!(1, campaigns.len());
  assert_eq!(
    campaigns[0].as_str(),
    campaign_account_id.as_str() // new_campaign.alice_linkdrop
  );
}
