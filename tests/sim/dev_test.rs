#![allow(
  unused_must_use,
  unused_imports,
  unused_mut,
  dead_code,
  unused_variables
)]
use crate::utils::{deploy_user_contract, get_contract_account, init_simulation, KeySet};
use near_campaign::CampaignContract;
use near_crypto::{InMemorySigner, SecretKey, Signer};
use near_sdk::json_types::U128;
use near_sdk::{AccountId, Balance, PublicKey};
use near_sdk_sim::{call, to_yocto, view};
use std::str::FromStr;
use user::UserContract;

// TODO Delete this file after all features will be done

const PK: &str = "8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe";
const SK: &str =
  "39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y";

#[test]
fn failed_campaign_creation() {
  let (root, runtime) = init_simulation();
  let bob = deploy_user_contract(&root, "bob");
  let alice = deploy_user_contract(&root, "alice");

  {
    let res = runtime.borrow().view_account(alice.account_id().as_str());
    dbg!(res);
  }
  // Create a new campaign
  let call1 = call!(
    alice.user_account,
    alice.create_near_campaign(
      "bob".to_string(),
      PublicKey::from_str(PK).unwrap(),
      10,
      U128::from(to_yocto("1")),
      "testnet-1".parse().unwrap()
    ),
    deposit = to_yocto("1")
  );
  dbg!(call1);

  {
    let res = runtime.borrow().view_account(alice.account_id().as_str());
    dbg!(res);
  }
}

#[test]
fn dev_test() {
  let (root, runtime) = init_simulation();
  let user = deploy_user_contract(&root, "bob");

  let view1 = view!(user.get_user_metadata());
  dbg!(view1.unwrap_json_value());

  // Create a new campaign
  // let call1 = call!(
  //   user.user_account,
  //   user.create_near_campaign(
  //     "my_campaign".to_string(),
  //     PublicKey::from_str(PK).unwrap(),
  //     5,
  //     U128::from(to_yocto("1")),
  //     "testnet-1".parse().unwrap()
  //   ),
  //   deposit = to_yocto("50")
  // );
  // Check user campaigns
  // let view1 = view!(user.get_campaigns());
  // dbg!(view1.unwrap_json_value());


  // let my_campaign = get_contract_account(
  //   "my_campaign.bob",
  //   SK,
  //   runtime.clone(),
  //   CampaignContract {
  //     account_id: "my_campaign.bob".parse().unwrap(),
  //   },
  // );
  // let view1 = view!(my_campaign.get_campaign_metadata());
  // dbg!(view1.unwrap_json_value());

  // Add keys
  // let key_set = KeySet::create(0, 4);
  //
  // let add_keys_result = call!(
  //   my_campaign.user_account,
  //   my_campaign.add_keys(key_set.public_keys())
  // );


  // Refund keys
  // let refund_keys_result = call!(
  //   my_campaign.user_account,
  //   my_campaign.refund_keys(key_set.public_keys(), user.account_id())
  // );
  // dbg!(refund_keys_result);
  // let view2 = view!(my_campaign.get_campaign_metadata());
  // dbg!(view2.unwrap_json_value());


  // Try to provoke an error by refunding already refunded keys
  // let refund_keys_result2 = call!(
  //   my_campaign.user_account,
  //   my_campaign.refund_keys(key_set.public_keys(), user.account_id())
  // );
  // dbg!(refund_keys_result2.promise_errors());


  // Delete Campaign
  // let delete_campaign_result = call!(
  //   my_campaign.user_account,
  //   my_campaign.delete_campaign(user.account_id())
  // );
  // dbg!(delete_campaign_result);
  // let view2 = view!(user.get_campaigns());
  // dbg!(view2.unwrap_json_value());
}
