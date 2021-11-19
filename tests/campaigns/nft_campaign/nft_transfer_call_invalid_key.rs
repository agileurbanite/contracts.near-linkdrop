use crate::utils::{CommonUtils, NftCampaignUtility, NftFactory};
use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;

#[test]
fn nft_transfer_call_invalid_key() {
  let nft_campaign_utils = NftCampaignUtility::init_nft_campaign("nft_campaign", 0, 0);
  let alice = nft_campaign_utils.create_user("alice");
  let nft_factory = NftFactory::default_init(nft_campaign_utils.root_account.clone(), "alice");
  nft_factory.default_nft_mint(&alice);
  let nft_campaign = nft_campaign_utils.contract;
  let campaign_balance_start = nft_campaign.account().unwrap().amount;

  let result = alice.call(
    nft_factory.account.account_id(),
    "nft_transfer_call",
    json!({
        "receiver_id": "nft_campaign",
        "token_id": "1",
        "msg": "0xffffff".to_string(),
      })
      .to_string()
      .as_bytes(),
    DEFAULT_GAS,
    1,
  );
  result.assert_success();

  // One error must occur while running the method
  CommonUtils::assert_one_promise_error(result.clone(), "Invalid key");

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("Return token 1 from @nft_campaign to @alice"));

  // The owner has not changed
  let nft = nft_factory.get_nft_token("1");
  assert_eq!(nft.as_object().unwrap().get("owner_id").unwrap(), "alice");

  // The Alice balance has not changed
  let runtime = nft_campaign_utils.runtime;
  let campaign_balance_end = CommonUtils::retrieve_account_balance(nft_campaign.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(campaign_balance_start, campaign_balance_end);
}