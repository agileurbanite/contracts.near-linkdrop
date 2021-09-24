use crate::utils::{assert_almost_eq_with_max_delta, init_user_and_near_campaign, KeySet};
use near_sdk::AccountId;
use near_sdk::serde_json::json;
use near_sdk_sim::{view, to_yocto, DEFAULT_GAS};

#[test]
fn delete_campaign_with_active_keys() {
  let initial_user_balance = to_yocto("700");
  let initial_campaign_balance = to_yocto("500");

  let (public_key, _, secret_key) = KeySet::create(0, 0).some_keys(0);
  let (root, user, near_campaign) = init_user_and_near_campaign(
    initial_user_balance,
    initial_campaign_balance,
    100,
    "5",
    public_key,
    secret_key
  );
  let campaign_account_id = near_campaign.account_id.as_str();

  let key_set = KeySet::create(1, 100);
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

  // Delete campaign
  let result = near_campaign.call(
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
  result.assert_success();

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
      to_yocto("700"),
      user_balance,
      to_yocto("0.2")
    );

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("Is campaign deleted: true"));

    let campaigns: Vec<AccountId> = view!(user.get_campaigns()).unwrap_json();
    assert_eq!(0, campaigns.len());
  }
}
