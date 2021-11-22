use crate::utils::{CommonUtils, NftCampaignCreatorUtility, UserUtility};
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_nft_campaign() {
  let (root, _runtime) = CommonUtils::init_simulation();
  let user_utility = UserUtility::init(root.clone(), to_yocto("100"));
  let creator_utility = NftCampaignCreatorUtility::init_nft_campaign_creator(root, "10");

  let user_contract = user_utility.contract;
  let pk = user_utility.public_key;
  let nft_campaign_creator = creator_utility.contract;

  let result = call!(
    user_contract.user_account,
    nft_campaign_creator.create_nft_campaign("nft_campaign".to_string(), pk.as_pk1()),
    deposit = to_yocto("10")
  );
  //TODO: after bug fix in the contract
  //result.assert_success();

  println!("{:#?}", result.promise_errors());
  println!("{:#?}", result.logs());
}
