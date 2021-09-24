use crate::utils::{assert_almost_eq_with_max_delta, init_user_and_near_campaign, KeySet};
use near_sdk::AccountId;
use near_sdk::serde_json::json;
use near_sdk_sim::{view, to_yocto, DEFAULT_GAS};

#[test]
fn add_keys_clear_and_delete_campaign() {
  let initial_user_balance = to_yocto("200");
  let initial_campaign_balance = to_yocto("50");

  let (public_key, _, secret_key) = KeySet::create(0, 0).some_keys(0);
  let (root, user, near_campaign) = init_user_and_near_campaign(
    initial_user_balance,
    initial_campaign_balance,
    10,
    "5",
    public_key,
    secret_key
  );
  let campaign_account_id = near_campaign.account_id.as_str();

  let key_set = KeySet::create(1, 10);
  near_campaign.call(
    near_campaign.account_id.clone(),
    "add_keys",
    &json!({
      "keys": key_set.public_keys()
    })
    .to_string()
    .into_bytes(),
    DEFAULT_GAS,
    0, // deposit
  );

  // Clear storage
  let clear_result = near_campaign.call(
    near_campaign.account_id.clone(),
    "clear_state",
    &json!({
      "keys": key_set.public_keys()
    })
    .to_string()
    .into_bytes(),
    DEFAULT_GAS,
    0, // deposit
  );
  clear_result.assert_success();

  {
    let runtime = root.borrow_runtime();
    let campaign_account = runtime.view_account(campaign_account_id);
    assert!(campaign_account.is_some());

    let campaigns: Vec<AccountId> = view!(user.get_campaigns()).unwrap_json();
    assert_eq!(1, campaigns.len());
  }

  // Delete campaign
  let delete_result = near_campaign.call(
    near_campaign.account_id.clone(),
    "delete_campaign",
    &json!({
      "beneficiary_id": user.account_id()
    })
    .to_string()
    .into_bytes(),
    DEFAULT_GAS,
    0, // deposit
  );
  delete_result.assert_success();

  {
    let runtime = root.borrow_runtime();

    let campaign_account = runtime.view_account(campaign_account_id);
    assert!(campaign_account.is_none());

    // Check User balance. The company's funds are returned to the user's contract
    let user_balance = runtime
      .view_account(user.account_id().as_str())
      .unwrap()
      .amount;
    assert_almost_eq_with_max_delta(
      to_yocto("200"),
      user_balance,
      to_yocto("0.2")
    );

    // Check the log for callback output
    assert_eq!(delete_result.logs().len(), 1);
    assert!(delete_result.logs()[0].contains("Is campaign deleted: true"));

    let campaigns: Vec<AccountId> = view!(user.get_campaigns()).unwrap_json();
    assert_eq!(0, campaigns.len());
  }
}
