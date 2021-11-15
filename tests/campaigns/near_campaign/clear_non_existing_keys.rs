use crate::utils::{CommonUtils, NearCampaignUtility};
use near_campaign::clear_state::ClearStatus;
use near_campaign::get_keys::Key;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};

#[test]
fn clear_non_existing_keys() {
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

  // Clear one key (sixth)
  let (pk_some, _) = keys.some_keys(5);
  let mut result = call!(
    contract.user_account,
    contract.clear_state(vec![pk_some.as_pk1()]),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  // Clear ten keys (one of which has already been deleted)
  result = call!(
    contract.user_account,
    contract.clear_state(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  // The first attempt cleared one key
  let mut key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_some.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // The second attempt cleared the remaining keys
  let (pk_first, _) = keys.some_keys(0);
  key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_first.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  let (pk_last, _) = keys.some_keys(9);
  key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_last.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check cleaning status
  let clear_status: ClearStatus = result.unwrap_json();
  match clear_status {
    ClearStatus::Completed(status) => assert_eq!(status, true),
  };

  // All keys have no status
  let keys: Vec<Key> = view!(contract.get_keys(keys.public_keys())).unwrap_json();
  assert_eq!(10, keys.len());
  assert_eq!(
    keys
      .into_iter()
      .find(|k| k.status.is_some())
      .is_none(),
    true
  );
}
