use crate::utils::{
  get_account_access_key,
  get_account_balance,
  NearCampaignUtility
};
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_one_account_and_claim() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    1,
    "5",
    0,
    1
  );
  near_campaign_utility.init_external_linkdrop();
  // Add first key for claim
  let (pk, _) = near_campaign_utility.keys.some_keys(0);
  near_campaign_utility.add_keys_from_array(vec![pk.as_pk1()]);
  near_campaign_utility.set_signer_to_claim(0);
  // The key to a new account
  let (new_pk, _) = near_campaign_utility.keys.some_keys(1);

  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;

  // Create a new account
  let result = call!(
    contract.user_account,
    contract.create_account_and_claim("john.testnet".parse().unwrap(), new_pk.as_pk1()),
    gas = 100000000000000 // 100 TGas
  );

  // The new account should exist with 5 NEAR on the balance
  let john_balance = get_account_balance("john.testnet", &runtime);
  assert_eq!(to_yocto("5"), john_balance);

  // Verify that the key has been added to the new account
  let johns_key = get_account_access_key("john.testnet", new_pk.as_pk2(), &runtime);
  assert_eq!(johns_key.is_some(), true);

  // Used key should not exist after the successful 'claim'
  let key = get_account_access_key(contract.account_id().as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_none(), true);

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("The account is created and link is claimed: true"));
}
