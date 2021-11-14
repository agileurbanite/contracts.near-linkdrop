use crate::utils::{
  assert_eq_with_gas,
  assert_one_promise_error,
  get_account_access_key,
  get_account_balance,
  NearCampaignUtility
};
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_account_and_claim_that_already_exists() {
  let mut near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    2,
    "5",
    0,
    2
  );
  near_campaign_utility.init_external_linkdrop();
  // Add two keys for claim drops
  let (pk1, _) = near_campaign_utility.keys.some_keys(0);
  let (pk2, _) = near_campaign_utility.keys.some_keys(1);
  near_campaign_utility.add_keys_from_array(vec![pk1.as_pk1(), pk2.as_pk1()]);
  // The key to a new account
  let (new_pk, _) = near_campaign_utility.keys.some_keys(2);

  // Create a new account for John
  near_campaign_utility.pre_create_account_and_claim("john.testnet".parse().unwrap(), 0, 2);

  near_campaign_utility.set_signer_to_claim(1);
  let runtime = near_campaign_utility.runtime;
  let contract = near_campaign_utility.contract;
  let campaign_balance_start = contract.account().unwrap().amount;

  // Let's try to repeat for an existing account
  let result = call!(
    contract.user_account,
    contract.create_account_and_claim("john.testnet".parse().unwrap(), new_pk.as_pk1()),
    gas = 100000000000000 // 100 TGas
  );
  result.assert_success();

  // One error must occur while running the method
  assert_one_promise_error(
    result.clone(),
    "Can't create a new account \"john.testnet\", because it already exists"
  );

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("The account is created and link is claimed: false"));

  // The balance of the contract has not changed
  let campaign_balance_end = get_account_balance(contract.account_id().as_str(), &runtime);
  assert_eq_with_gas(campaign_balance_start, campaign_balance_end);

  // The last key should not be deleted
  let key = get_account_access_key(contract.account_id().as_str(), pk2.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);
}
