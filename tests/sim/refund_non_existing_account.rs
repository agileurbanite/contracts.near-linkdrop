use crate::utils::{assert_almost_eq_with_max_delta, init_near_campaign, KeySet};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn refund_non_existing_account() {
  let (root, near_campaign) = init_near_campaign(10, "5");
  let key_set = KeySet::create(0, 9);
  let (_, pk_first, _) = key_set.some_keys(0);
  let (_, pk_last, _) = key_set.some_keys(9);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  let campaign_balance_start = near_campaign.account().unwrap().amount;

  // Tokens refund
  let result = call!(
    near_campaign.user_account,
    near_campaign.refund_keys(key_set.public_keys(), "alice".parse().unwrap()),
    gas = 3 * DEFAULT_GAS
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

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
    let campaign_balance_end = runtime
      .view_account(near_campaign.account_id().as_str())
      .unwrap()
      .amount;
    assert_almost_eq_with_max_delta(
      campaign_balance_start,
      campaign_balance_end,
      to_yocto("0.02")
    );

    // Check first Campaign access key
    let mut key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_first);
    assert_eq!(key.is_some(), true);

    // Check last Campaign access key
    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_last);
    assert_eq!(key.is_some(), true);

    // Check key statuses
    let metadata: Metadata = view!(near_campaign.get_campaign_metadata()).unwrap_json();
    assert_eq!(10, metadata.keys_stats.active);
    assert_eq!(0, metadata.keys_stats.refunded);
  }
}
