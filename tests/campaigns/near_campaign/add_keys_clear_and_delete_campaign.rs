use crate::utils::{
  assert_almost_eq_with_max_delta,
  get_account_balance,
  NearCampaignUtility
};
use near_sdk::AccountId;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn add_keys_clear_and_delete_campaign() {
  let initial_user_balance = to_yocto("200");
  let initial_campaign_balance = to_yocto("50");

  let mut near_campaign_utility = NearCampaignUtility::init_user_and_near_campaign(
    initial_user_balance,
    initial_campaign_balance,
    10,
    "5",
    1,
    10
  );
  near_campaign_utility.add_keys();

  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let user = near_campaign_utility.user_contract.unwrap();
  let keys = near_campaign_utility.keys;

  // Clear storage
  let clear_result = call!(
    contract.user_account,
    contract.clear_state(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  clear_result.assert_success();

  let campaign_account = runtime.borrow().view_account(contract.account_id().as_str());
  assert!(campaign_account.is_some());

  let campaigns: Vec<AccountId> = view!(user.get_campaigns()).unwrap_json();
  assert_eq!(1, campaigns.len());

  // Delete campaign
  let delete_result = call!(
    contract.user_account,
    contract.delete_campaign(user.account_id()),
    gas = DEFAULT_GAS
  );
  delete_result.assert_success();

  let campaign_account = runtime.borrow().view_account(contract.account_id().as_str());
  assert!(campaign_account.is_none());

  // Check User balance. The company's funds are returned to the user's contract
  let user_balance = get_account_balance(user.account_id().as_str(), &runtime);
  assert_almost_eq_with_max_delta(
    to_yocto("200"),
    user_balance,
    to_yocto("0.2")
  );

  // Check the log for callback output
  assert_eq!(delete_result.logs().len(), 1);
  assert!(delete_result.logs()[0].contains("Is campaign deleted: true"));

  let campaigns: Vec<AccountId> = view!(user.get_campaigns()).unwrap_json();
  assert_eq!(0, campaigns.len());
}
