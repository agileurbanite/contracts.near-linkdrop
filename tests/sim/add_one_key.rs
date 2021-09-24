use crate::utils::{init_near_campaign, KeySet};
use near_sdk_sim::account::AccessKeyPermission;
use near_sdk_sim::call;

#[test]
fn add_one_key() {
  let (root, near_campaign) = init_near_campaign(1, "5");
  let key_set = KeySet::create(0, 0);
  let (_, pk, _) = key_set.some_keys(0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys())
  );

  {
    let runtime = root.borrow_runtime();

    let key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk);
    assert_eq!(key.is_some(), true);

    // Was the key added as a functional call assess key ?
    matches!(
      key.unwrap().permission,
      AccessKeyPermission::FunctionCall(_)
    );
  }
}
