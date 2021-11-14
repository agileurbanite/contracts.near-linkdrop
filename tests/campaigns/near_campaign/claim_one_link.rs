use crate::utils::{
  get_account_access_key,
  NearCampaignUtility
};
use near_sdk_sim::{call, to_yocto};

#[test]
fn claim_one_link() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    1,
    "5",
    0,
    0
  );
  near_campaign_utility.add_keys();
  near_campaign_utility.set_signer_to_claim(0);
  let alice = near_campaign_utility.create_user(to_yocto("10"));
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let keys = near_campaign_utility.keys;

  let result = call!(
    contract.user_account,
    contract.claim(alice.account_id()),
    gas = 100000000000000 // 100 TGas
  );
  result.assert_success();

  // Used key should not exist after the successful 'claim'
  let (pk, _) = keys.some_keys(0);
  let key = get_account_access_key(contract.account_id().as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check Alice balance
  assert_eq!(to_yocto("15"), alice.account().unwrap().amount);

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("The link is claimed: true"));
}
