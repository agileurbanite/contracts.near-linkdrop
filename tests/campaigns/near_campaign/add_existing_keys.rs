use crate::utils::{CommonUtils, NearCampaignUtility};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};

#[test]
fn add_existing_keys() {
  let near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    10,
    "5",
    0,
    9
  );
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  // Add one key
  let (pk_some, _) = keys.some_keys(5);
  call!(
    contract.user_account,
    contract.add_keys(vec![pk_some.as_pk1()]),
    gas = DEFAULT_GAS
  );

  // Add ten keys
  let result = call!(
    contract.user_account,
    contract.add_keys(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  // One error must occur on the second attempt
  CommonUtils::assert_one_promise_error(result.clone(), "Key is already exists");

  // The first attempt added one key
  let mut key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_some.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);

  // The second attempt should not add any keys
  let (pk_first, _) = keys.some_keys(0);
  key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_first.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  let (pk_last, _) = keys.some_keys(9);
  key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_last.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check the state of the contract
  let metadata: Metadata = view!(contract.get_campaign_metadata()).unwrap_json();
  assert_eq!(1, metadata.keys_stats.active);
}
