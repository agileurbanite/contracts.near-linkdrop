use crate::utils::{
  assert_eq_with_gas,
  assert_one_promise_error,
  init_external_linkdrop,
  init_near_campaign,
  KeySet
};
use near_crypto::{InMemorySigner, Signer};
use near_sdk_sim::call;

#[test]
fn create_account_and_claim_that_already_exists() {
  let (root, mut near_campaign) = init_near_campaign(2, "5");
  init_external_linkdrop(&root);

  let key_set = KeySet::create(0, 1);
  let (_, _, sk1) = key_set.some_keys(0);
  let (_, _, sk2) = key_set.some_keys(1);
  let (new_pk, _, _) = KeySet::create(2, 2).some_keys(0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys())
  );

  // We want to sing the first transaction on a new key
  let mut claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk1);
  near_campaign.user_account.signer = claim_signer.clone();

  // Create a new account for John
  call!(
    near_campaign.user_account,
    near_campaign.create_account_and_claim("john.testnet".parse().unwrap(), new_pk.clone())
  );
  let campaign_balance_start = near_campaign.account().unwrap().amount;

  // The key for the next transaction
  claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk2);
  near_campaign.user_account.signer = claim_signer.clone();

  // Let's try to repeat for an existing account
  let result = call!(
    near_campaign.user_account,
    near_campaign.create_account_and_claim("john.testnet".parse().unwrap(), new_pk)
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // One error must occur while running the method
    assert_one_promise_error(
      result.clone(),
      "Can't create a new account \"john.testnet\", because it already exists"
    );

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("The account is created and link is claimed: false"));

    // The balance of the contract has not changed
    let campaign_balance_end = runtime
      .view_account(near_campaign.account_id().as_str())
      .unwrap()
      .amount;
    assert_eq_with_gas(campaign_balance_start, campaign_balance_end);

    // The last key should not be deleted
    let key = runtime.view_access_key(
      near_campaign.account_id().as_str(),
      &claim_signer.public_key(),
    );
    assert_eq!(key.is_some(), true);
  }
}
