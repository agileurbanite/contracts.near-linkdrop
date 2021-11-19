use crate::utils::{CommonUtils, NftCampaignUtility, NftFactory};
use near_sdk::serde_json::json;
use near_sdk_sim::{view, DEFAULT_GAS};
use nft_campaign::{DropStatus, Drop};

#[test]
fn nft_transfer_call() {
  let nft_campaign_utils = NftCampaignUtility::init_nft_campaign("nft_campaign", 0, 0);
  let (pk, _) = nft_campaign_utils.keys.some_keys(0);

  let alice = nft_campaign_utils.create_user("alice");
  let nft_factory = NftFactory::default_init(nft_campaign_utils.root_account.clone(), "alice");
  nft_factory.default_nft_mint(&alice);
  let nft_campaign = nft_campaign_utils.contract;

  // Check NFT Metadata
  let metadata = nft_factory.get_nft_metadata();
  assert!(metadata.as_object().unwrap().get("name").is_some());

  // Check current owner
  let nft = nft_factory.get_nft_token("1");
  assert_eq!(nft.as_object().unwrap().get("owner_id").unwrap(), "alice");

  let result = alice.call(
    nft_factory.account.account_id(),
    "nft_transfer_call",
    json!({
        "receiver_id": "nft_campaign",
        "token_id": "1",
        "msg": pk.as_pk2().to_string(),
      })
      .to_string()
      .as_bytes(),
    DEFAULT_GAS,
    1,
  );
  result.assert_success();

  // NFT Owner Change Check
  let nft = nft_factory.get_nft_token("1");
  assert_eq!(nft.as_object().unwrap().get("owner_id").unwrap(), "nft_campaign");

  // Check the Drop status
  let drops: Vec<Option<Drop>> = view!(nft_campaign.get_drops(vec![pk.as_pk1()])).unwrap_json();
  assert_eq!(drops.len(), 1);
  match drops.get(0).unwrap() {
    Some(drop) => {
      assert_eq!(drop.status, DropStatus::ACTIVE);
      assert_eq!(drop.nft.token_id, "1");
    },
    _ => panic!("Drop not found"),
  }

  let runtime = nft_campaign_utils.runtime;
  let key = CommonUtils::retrieve_account_access_key(nft_campaign.account_id().as_str(), pk.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);
}