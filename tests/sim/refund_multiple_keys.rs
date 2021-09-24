use crate::utils::{
  assert_almost_eq_with_max_delta, assert_eq_with_gas, init_near_campaign, KeySet,
};
use near_campaign::get_campaign_metadata::Metadata;
use near_sdk::AccountId;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn refund_multiple_keys() {
  let tera_gas = u64::pow(10, 12);
  let expected_gas_ceiling: u64 = tera_gas * 120; // 120 TeraGas

  let (root, near_campaign) = init_near_campaign(10, "5");
  let key_set = KeySet::create(0, 9);
  let (_, pk_first, _) = key_set.some_keys(0);
  let (_, pk_last, _) = key_set.some_keys(9);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );

  // Create beneficiary account
  let alice = root.create_user(
    AccountId::new_unchecked("alice".to_string()),
    to_yocto("10"),
  );

  // Tokens refund
  let result = call!(
    near_campaign.user_account,
    near_campaign.refund_keys(key_set.public_keys(), alice.account_id),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // Check Alice balance
    let alice_balance = runtime.view_account("alice").unwrap().amount;
    assert_eq_with_gas(
      to_yocto("60"), // 10 + 10 * 5 NEAR
      alice_balance,
    );

    // Check Campaign balance
    let campaign_balance = runtime
      .view_account(near_campaign.account_id().as_str())
      .unwrap()
      .amount;
    assert_almost_eq_with_max_delta(
      to_yocto("150"), // 200 - 10 * 5 NEAR
      campaign_balance,
      to_yocto("0.02"),
    );

    // Check first Campaign access key
    let mut key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_first);
    assert_eq!(key.is_none(), true);

    // Check last Campaign access key
    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_last);
    assert_eq!(key.is_none(), true);

    // Check key statuses
    let metadata: Metadata = view!(near_campaign.get_campaign_metadata()).unwrap_json();
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
}
