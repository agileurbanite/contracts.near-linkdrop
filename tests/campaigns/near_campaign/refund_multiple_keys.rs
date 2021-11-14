use crate::utils::{
  assert_eq_with_gas,
  get_account_balance,
  get_account_access_key,
  NearCampaignUtility
};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn refund_multiple_keys() {
  let tera_gas = u64::pow(10, 12);
  let expected_gas_ceiling: u64 = tera_gas * 140; // 140 TeraGas

  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    10,
    "5",
    0,
    9
  );
  near_campaign_utility.add_keys();

  // Create beneficiary account
  let alice = near_campaign_utility.create_user(to_yocto("10"));

  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  // Tokens refund
  let result = call!(
    contract.user_account,
    contract.refund_keys(keys.public_keys(), alice.account_id),
    gas = 3 * DEFAULT_GAS
  );
  result.assert_success();

  // Check Alice balance
  let alice_balance = get_account_balance("alice", &runtime);
  assert_eq_with_gas(
    to_yocto("60"), // 10 + 10 * 5 NEAR
    alice_balance,
  );

  // Check Campaign balance
  let campaign_balance = get_account_balance(contract.account_id().as_str(), &runtime);
  assert_eq_with_gas(
    to_yocto("150"), // 200 - 10 * 5 NEAR
    campaign_balance
  );

  // Check first Campaign access key
  let (pk_first, _) = keys.some_keys(0);
  let mut key = get_account_access_key(contract.account_id().as_str(), pk_first.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check last Campaign access key
  let (pk_last, _) = keys.some_keys(9);
  key = get_account_access_key(contract.account_id().as_str(), pk_last.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check key statuses
  let metadata: Metadata = view!(contract.get_campaign_metadata()).unwrap_json();
  assert_eq!(0, metadata.keys_stats.active);
  assert_eq!(10, metadata.keys_stats.refunded);

  // Check TeraGas burnt
  println!(
    "refund_multiple_keys > TeraGas burnt: {}",
    result.gas_burnt().0 as f64 / 1e12
  );
  println!(
    "refund_multiple_keys > Tokens burnt: {}",
    result.tokens_burnt() as f64 / 1e24
  );
  assert!(result.gas_burnt().0 < expected_gas_ceiling);
}
