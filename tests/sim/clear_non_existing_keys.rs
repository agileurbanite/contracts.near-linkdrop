use crate::utils::{init_near_campaign, KeySet};
use near_sdk_sim::{call, view, DEFAULT_GAS};

#[test]
fn clear_non_existing_keys() {
  let (root, near_campaign) = init_near_campaign(10, "5");
  let key_set = KeySet::create(0, 9);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );

  // Clear one key (sixth)
  let one_key = KeySet::create(5, 5);
  let (_, pk_some, _) = one_key.some_keys(0);
  let mut result = call!(
    near_campaign.user_account,
    near_campaign.clear_state(one_key.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  // Clear ten keys (one of which has already been deleted)
  let (_, pk_first, _) = key_set.some_keys(0);
  let (_, pk_last, _) = key_set.some_keys(9);
  result = call!(
    near_campaign.user_account,
    near_campaign.clear_state(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // The first attempt cleared one key
    let mut key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_some);
    assert_eq!(key.is_none(), true);

    // The second attempt cleared the remaining keys
    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_first);
    assert_eq!(key.is_none(), true);

    key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk_last);
    assert_eq!(key.is_none(), true);

    // Check cleaning status
    let mut value = result.unwrap_json_value();
    let clear_status = value.as_object().unwrap().get("Completed").unwrap().as_bool().unwrap();
    assert_eq!(clear_status, true);

    // All keys have no status
    value = view!(near_campaign.get_keys(key_set.public_keys())).unwrap_json_value();
    let keys = value
      .as_array()
      .unwrap();
    assert_eq!(10, keys.len());
    keys
      .into_iter()
      .for_each(|v| {
        let status = v
          .as_object()
          .unwrap()
          .get("status")
          .unwrap();
        assert_eq!(status.is_null(), true);
      });
  }
}
