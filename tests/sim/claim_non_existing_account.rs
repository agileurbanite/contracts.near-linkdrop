use crate::utils::{
  assert_eq_with_gas,
  assert_one_promise_error,
  init_near_campaign,
  KeySet
};
use near_crypto::{InMemorySigner, Signer};
use near_sdk::serde_json::json;
use near_sdk_sim::call;

#[test]
fn claim_non_existing_account() {
  let (root, mut near_campaign) = init_near_campaign(1, "5");
  let key_set = KeySet::create(0, 0);
  let (_, _, sk) = key_set.some_keys(0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys())
  );

  // We want to sing transaction by new key;
  let claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk);
  near_campaign.user_account.signer = claim_signer.clone();

  let campaign_balance_start = near_campaign.account().unwrap().amount;
  let result = near_campaign.user_account.call(
    near_campaign.account_id().clone(),
    "claim",
    &json!({
      "account_id": "bob".to_string()
    })
      .to_string()
      .into_bytes(),
    100000000000000, // 100 TGas
    0
  );
  result.assert_success();

  {
    let runtime = root.borrow_runtime();

    // One error must occur while running the method
    assert_one_promise_error(
      result.clone(),
      "Can't complete the action because account \"bob\" doesn't exist"
    );

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("The link is claimed: false"));

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
