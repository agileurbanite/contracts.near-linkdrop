use crate::utils::{get_account_access_key, NearCampaignUtility};
use near_sdk_sim::account::AccessKeyPermission;
use near_sdk_sim::{call, to_yocto};

#[test]
fn add_one_key() {
  let near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    1,
    "5",
    0,
    0
  );
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  call!(
    contract.user_account,
    contract.add_keys(keys.public_keys())
  );

  let (pk, _) = keys.some_keys(0);
  let key = get_account_access_key(contract.account_id().as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);

  // Was the key added as a functional call assess key ?
  matches!(
      key.unwrap().permission,
      AccessKeyPermission::FunctionCall(_)
  );
}
