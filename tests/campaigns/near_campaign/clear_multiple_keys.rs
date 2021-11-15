use crate::utils::{CommonUtils, NearCampaignUtility};
use near_campaign::clear_state::ClearStatus;
use near_campaign::get_keys::Key;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};

#[test]
fn clear_multiple_keys() {
  let tera_gas = u64::pow(10, 12);
  let expected_gas_ceiling: u64 = tera_gas * 160; // 160 TeraGas

  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    100,
    "5",
    0,
    99
  );
  near_campaign_utility.add_keys();
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  // Clear storage
  let result = call!(
    contract.user_account,
    contract.clear_state(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  // Check first Campaign access key
  let (pk_first, _) = keys.some_keys(0);
  let mut key = CommonUtils::retrieve_account_access_key(contract.account_id().as_str(), pk_first.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check last Campaign access key
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
  assert_eq!(100, keys.len());
  assert_eq!(
    keys
      .into_iter()
      .find(|k| k.status.is_some())
      .is_none(),
    true
  );

  // Check TeraGas burnt
  println!(
    "clear_multiple_keys > TeraGas burnt: {}",
    result.gas_burnt().0 as f64 / 1e12
  );
  println!(
    "clear_multiple_keys > Tokens burnt: {}",
    result.tokens_burnt() as f64 / 1e24
  );
  assert!(result.gas_burnt().0 < expected_gas_ceiling);
}
