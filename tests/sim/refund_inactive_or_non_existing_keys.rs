use crate::utils::{
  assert_almost_eq_with_max_delta,
  assert_eq_with_gas,
  assert_one_promise_error,
  init_near_campaign,
  KeySet
};
use near_campaign::get_campaign_metadata::Metadata;
use near_crypto::{InMemorySigner};
use near_sdk::AccountId;
use near_sdk_sim::{call, view, to_yocto, DEFAULT_GAS};

#[test]
fn refund_inactive_or_non_existing_keys() {
  let (root, mut near_campaign) = init_near_campaign(10, "5");
  let key_set = KeySet::create(0, 9);

  // Create beneficiary account
  let alice = root.create_user(
    AccountId::new_unchecked("alice".to_string()),
    to_yocto("10"),
  );

  // Add 10 keys
  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );

  // Claim one link (sixth)
  let (_, _, sk) = key_set.some_keys(5);
  let claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk);
  let old_signer = near_campaign.user_account.signer;
  near_campaign.user_account.signer = claim_signer.clone();
  call!(
    near_campaign.user_account,
    near_campaign.claim(alice.account_id())
  );
  assert_eq!(to_yocto("15"), alice.account().unwrap().amount);
  assert_eq_with_gas(to_yocto("195"), near_campaign.account().unwrap().amount);
  near_campaign.user_account.signer = old_signer;

  // Attempt to refund 10 keys
  let result = call!(
    near_campaign.user_account,
    near_campaign.refund_keys(key_set.public_keys(), alice.account_id),
    gas = 2 * DEFAULT_GAS
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  {
    let runtime = root.borrow_runtime();

    // One error must occur while running the method
    assert_one_promise_error(
      result.clone(),
      "Cannot refund inactive or non-existing key"
    );

    // The balance of the company and Alice remained unchanged
    let alice_balance = runtime.view_account("alice").unwrap().amount;
    assert_eq!(to_yocto("15"), alice_balance);

    let campaign_balance = runtime.view_account("near_campaign").unwrap().amount;
    assert_almost_eq_with_max_delta(
      to_yocto("195"),
      campaign_balance,
      to_yocto("0.01")
    );

    // Key statuses must not change
    let metadata: Metadata = view!(near_campaign.get_campaign_metadata()).unwrap_json();
    assert_eq!(9, metadata.keys_stats.active);
    assert_eq!(1, metadata.keys_stats.claimed);
    assert_eq!(0, metadata.keys_stats.refunded);
  }
}
