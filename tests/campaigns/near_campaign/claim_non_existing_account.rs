use crate::utils::{CommonUtils, NearCampaignUtility};
use near_sdk_sim::{call, to_yocto};

#[test]
fn claim_non_existing_account() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    1,
    "5",
    0,
    0
  );
  near_campaign_utility.add_keys();
  near_campaign_utility.set_signer_to_claim(0);
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  let campaign_balance_start = contract.account().unwrap().amount;
  let result = call!(
    contract.user_account,
    contract.claim("bob".parse().unwrap()),
    gas = 100000000000000 // 100 TGas
  );
  result.assert_success();

  // One error must occur while running the method
  CommonUtils::assert_one_promise_error(
    result.clone(),
    "Can't complete the action because account \"bob\" doesn't exist"
  );

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("The link is claimed: false"));

  // The balance of the contract has not changed
  let campaign_balance_end = CommonUtils::retrieve_account_balance(contract.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(campaign_balance_start, campaign_balance_end);

  // The last key should not be deleted
  let (pk, _) = keys.some_keys(0);
  let key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);
}
