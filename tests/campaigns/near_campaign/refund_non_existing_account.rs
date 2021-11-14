use crate::utils::{
  assert_eq_with_gas,
  get_account_balance,
  get_account_access_key,
  NearCampaignUtility
};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};

#[test]
fn refund_non_existing_account() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    10,
    "5",
    0,
    9
  );
  near_campaign_utility.add_keys();
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  let campaign_balance_start = contract.account().unwrap().amount;

  // Tokens refund
  let result = call!(
    contract.user_account,
    contract.refund_keys(keys.public_keys(), "alice".parse().unwrap()),
    gas = 3 * DEFAULT_GAS
  );
  result.assert_success();

  // Promise error for each key
  assert_eq!(result.promise_errors().len(), 10);
  result.promise_errors().into_iter().for_each(|key| {
    if let ExecutionStatus::Failure(execution_error) = &key.unwrap().outcome().status {
      assert!(execution_error.to_string().contains(
        "Can't complete the action because account \"alice\" doesn't exist"
      ));
    } else {
      unreachable!();
    }
  });

  // The balance of the contract has not changed
  let campaign_balance_end = get_account_balance(contract.account_id().as_str(), &runtime);
  assert_eq_with_gas(campaign_balance_start, campaign_balance_end);

  // Check first Campaign access key
  let (pk_first, _) = keys.some_keys(0);
  let mut key = get_account_access_key(contract.account_id().as_str(), pk_first.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);

  // Check last Campaign access key
  let (pk_last, _) = keys.some_keys(9);
  key = get_account_access_key(contract.account_id().as_str(), pk_last.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);

  // Check key statuses
  let metadata: Metadata = view!(contract.get_campaign_metadata()).unwrap_json();
  assert_eq!(10, metadata.keys_stats.active);
  assert_eq!(0, metadata.keys_stats.refunded);
}
