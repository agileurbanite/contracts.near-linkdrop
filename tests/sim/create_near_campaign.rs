#![allow(
  unused_must_use,
  unused_imports,
  unused_mut,
  dead_code,
  unused_variables
)]
use crate::utils::{deploy_user_contract, get_contract_account, get_public_keys, init};
use near_crypto::{InMemorySigner, SecretKey, Signer};
use near_sdk::json_types::U128;
use near_sdk::{AccountId, Balance, PublicKey};
use near_sdk_sim::{call, to_yocto, view};
use std::str::FromStr;
use user::UserContract;
use near_campaign::CampaignContract;

// TODO Need to finish

const PK: &str = "8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe";
const SK: &str =
  "39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y";

#[test]
fn create_near_campaign() {
  let (root, runtime, _) = init("5");
  let user = deploy_user_contract(&root);

  let res1 = call!(
    user.user_account,
    user.create_near_campaign(
      "my_campaign".to_string(),
      PublicKey::from_str(PK).unwrap(),
      U128::from(to_yocto("1"))
    ),
    deposit = to_yocto("10")
  );

  let my_campaign = get_contract_account(
    "my_campaign.bob",
    runtime.clone(),
    CampaignContract { account_id: "my_campaign.bob".parse().unwrap() },
  );

  let res2 = call!(
    user.user_account,
    user.create_near_campaign(
      "other_campaign".to_string(),
      PublicKey::from_str(PK).unwrap(),
      U128::from(to_yocto("0.5"))
    ),
    deposit = to_yocto("10")
  );

  dbg!(res2);

  let other_campaign = get_contract_account(
    "other_campaign.bob",
    runtime.clone(),
    CampaignContract { account_id: "other_campaign.bob".parse().unwrap() },
  );

  let r1 = view!(my_campaign.get_campaign_metadata());
  dbg!(r1.unwrap_json_value());

  let r2 = view!(other_campaign.get_campaign_metadata());
  dbg!(r2.unwrap_json_value());
}
