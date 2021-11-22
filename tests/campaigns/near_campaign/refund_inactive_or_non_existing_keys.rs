use crate::utils::{CommonUtils, NearCampaignUtility};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn refund_inactive_or_non_existing_keys() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    10,
    "5",
    0,
    9
  );
  near_campaign_utility.add_keys();

  // Create beneficiary account
  let alice = near_campaign_utility.create_user("10");

  // Claim one link (sixth)
  near_campaign_utility.pre_claim_one_link(alice.account_id.clone(), 5);

  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  assert_eq!(to_yocto("15"), alice.account().unwrap().amount);
  CommonUtils::assert_eq_with_gas(to_yocto("195"), contract.account().unwrap().amount);

  // Attempt to refund 10 keys
  let result = call!(
    contract.user_account,
    contract.refund_keys(keys.public_keys(), alice.account_id),
    gas = 2 * DEFAULT_GAS
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  // One error must occur while running the method
  CommonUtils::assert_one_promise_error(
    result.clone(),
    "Cannot refund inactive or non-existing key"
  );

  // The balance of the company and Alice remained unchanged
  let alice_balance = CommonUtils::retrieve_account_balance("alice", &runtime);
  assert_eq!(to_yocto("15"), alice_balance);

  let campaign_balance = CommonUtils::retrieve_account_balance("near_campaign", &runtime);
  CommonUtils::assert_eq_with_gas(to_yocto("195"), campaign_balance);

  // Key statuses must not change
  let metadata: Metadata = view!(contract.get_campaign_metadata()).unwrap_json();
  assert_eq!(9, metadata.keys_stats.active);
  assert_eq!(1, metadata.keys_stats.claimed);
  assert_eq!(0, metadata.keys_stats.refunded);
}
