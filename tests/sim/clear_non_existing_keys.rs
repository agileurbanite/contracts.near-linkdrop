use crate::utils::{init_near_campaign, KeySet};
use near_campaign::clear_state::ClearStatus;
use near_campaign::get_keys::Key;
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
    let clear_status: ClearStatus = result.unwrap_json();
    match clear_status {
      ClearStatus::Completed(status) => assert_eq!(status, true),
    };

    // All keys have no status
    let keys: Vec<Key> = view!(near_campaign.get_keys(key_set.public_keys())).unwrap_json();
    assert_eq!(10, keys.len());
    assert_eq!(
      keys
        .into_iter()
        .find(|k| k.status.is_some())
        .is_none(),
      true
    );
  }
}
