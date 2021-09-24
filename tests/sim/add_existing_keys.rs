use crate::utils::{assert_one_promise_error, init_near_campaign, KeySet};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::{call, view, DEFAULT_GAS};

#[test]
fn add_existing_keys() {
  let (root, near_campaign) = init_near_campaign(10, "5");

  // Add one key
  let mut key_set = KeySet::create(5, 5);
  let (_, pk_some, _) = key_set.some_keys(0);
  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );

  // Add ten keys
  key_set = KeySet::create(0, 9);
  let (_, pk_first, _) = key_set.some_keys(0);
  let (_, pk_last, _) = key_set.some_keys(9);
  let result = call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  {
    let runtime = root.borrow_runtime();

    // One error must occur on the second attempt
    assert_one_promise_error(result.clone(), "Key is already exists");

    // The first attempt added one key
    let mut key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_some);
    assert_eq!(key.is_some(), true);

    // The second attempt should not add any keys
    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_first);
    assert_eq!(key.is_none(), true);

    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_last);
    assert_eq!(key.is_none(), true);

    // Check the state of the contract
    let metadata: Metadata = view!(near_campaign.get_campaign_metadata()).unwrap_json();
    assert_eq!(1, metadata.keys_stats.active);
  }
}
