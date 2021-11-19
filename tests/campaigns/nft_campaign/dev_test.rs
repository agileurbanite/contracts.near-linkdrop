use crate::utils::{NftCampaignUtility, NftFactory};
use near_sdk_sim::{call, view, DEFAULT_GAS};

#[test]
fn test() {
  let mut nft_campaign_utils = NftCampaignUtility::init_nft_campaign("nft_campaign", 0, 2);
  let (key, _) = nft_campaign_utils.keys.some_keys(0);
  let (key2, _) = nft_campaign_utils.keys.some_keys(1);

  let alice = nft_campaign_utils.create_user("alice");
  let nft_factory = NftFactory::default_init(nft_campaign_utils.root_account.clone(), "alice");
  nft_factory.default_nft_mint(&alice);
  nft_factory.nft_transfer_call(&alice, "nft_campaign", "1", key.as_pk2().to_string().as_str());

  // dbg!(nft_factory.get_nft_token("1"));

  // Call claim
  nft_campaign_utils.set_signer_to_claim(0);
  let nft_campaign = nft_campaign_utils.contract;

  let result2 = call!(
    nft_campaign.user_account,
    nft_campaign.claim("new_owner".parse().unwrap()),
    gas = DEFAULT_GAS
  );

  dbg!(&result2);
  // dbg!(&result2.promise_results());

  let view1 = view!(nft_campaign.get_drops(vec![key.as_pk1(), key2.as_pk1()]));
  dbg!(view1.unwrap_json_value());

  // Check key
  // {
  //   // let res = nft_campaign_utils.runtime.borrow().view_account(&nft.account_id().as_str());
  //   let key = nft_campaign_utils.runtime.borrow().view_access_key("nft_campaign", &key.as_pk2());
  //   dbg!(key);
  // }
}
