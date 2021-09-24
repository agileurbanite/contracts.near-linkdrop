use crate::utils::{init_near_campaign, KeySet};
use near_campaign::clear_state::ClearStatus;
use near_campaign::get_keys::Key;
use near_sdk_sim::{call, view, DEFAULT_GAS};

#[test]
fn clear_multiple_keys() {
  let tera_gas = u64::pow(10, 12);
  let expected_gas_ceiling: u64 = tera_gas * 160; // 160 TeraGas

  let (root, near_campaign) = init_near_campaign(100, "5");
  let key_set = KeySet::create(0, 99);
  let (_, pk_first, _) = key_set.some_keys(0);
  let (_, pk_last, _) = key_set.some_keys(9);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );

  // Clear storage
  let result = call!(
    near_campaign.user_account,
    near_campaign.clear_state(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // Check first Campaign access key
    let mut key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_first);
    assert_eq!(key.is_none(), true);

    // Check last Campaign access key
    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_last);
    assert_eq!(key.is_none(), true);

    // Check cleaning status
    let clear_status: ClearStatus = result.unwrap_json();
    match clear_status {
      ClearStatus::Completed(status) => assert_eq!(status, true),
    };

    // All keys have no status
    let keys: Vec<Key> = view!(near_campaign.get_keys(key_set.public_keys())).unwrap_json();
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
}
