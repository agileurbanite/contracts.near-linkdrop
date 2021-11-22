use crate::utils::{NftCampaignUtility, NftFactory};
use near_sdk_sim::{call, view, DEFAULT_GAS};
use nft_campaign::{DropStatus, Drop};

#[test]
fn claim_one_nft_non_existing_account() {
  let mut nft_campaign_utils = NftCampaignUtility::init_nft_campaign("nft_campaign", 0, 0);
  let (pk, _) = nft_campaign_utils.keys.some_keys(0);

  let alice = nft_campaign_utils.create_user("alice");
  let nft_factory = NftFactory::default_init(nft_campaign_utils.root_account.clone(), "alice");
  nft_factory.default_nft_mint(&alice);
  nft_factory.nft_transfer_call(&alice, "nft_campaign", "1", pk.as_pk2().to_string().as_str());

  nft_campaign_utils.set_signer_to_claim(0);
  let nft_campaign = nft_campaign_utils.contract;

  // Claim one NFT
  let result = call!(
    nft_campaign.user_account,
    nft_campaign.claim("new_owner".parse().unwrap()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  // TODO: Check the Drop status
  let drops: Vec<Option<Drop>> = view!(nft_campaign.get_drops(vec![pk.as_pk1()])).unwrap_json();
  assert_eq!(drops.len(), 1);
  match drops.get(0).unwrap() {
    Some(drop) => {
      assert_eq!(drop.status, DropStatus::CLAIMED); // Must be: ACTIVE
      assert_eq!(drop.nft.token_id, "1");
    },
    _ => panic!("Drop not found"),
  }

  //println!("{:#?}", result.logs());
  // TODO: Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("Successfully transfer token '1' of @nft_factory to @new_owner"));
  // Must be: "Failed to transfer token '{}' of @{} to @{}"

  // TODO: The owner has not changed
  let nft = nft_factory.get_nft_token("1");
  assert_eq!(nft.as_object().unwrap().get("owner_id").unwrap(), "new_owner"); // Must be: alice
}
