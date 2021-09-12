#![allow(
  unused_must_use,
  unused_imports,
  unused_mut,
  dead_code,
  unused_variables
)]
use crate::utils::{KeySet, init};
use near_crypto::{InMemorySigner, PublicKey, SecretKey, Signer};
use near_sdk_sim::{call, to_yocto, view};
use std::str::FromStr;

// TODO Need to finish
// Test the that the add_keys will works correctly with the predefined keys amount

#[test]
fn add_keys_fail() {
  let (root, _, mut near_campaign) = init("5");
  let public_keys = KeySet::create(0, 49).public_keys();

  let res = call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  // let res1 = view!(near_campaign.get_keys(get_public_keys(89, 99)));
  let res1 = view!(near_campaign.get_campaign_metadata());
  dbg!(res1.unwrap_json_value());

  // {
  //   let mut a = root.borrow_runtime_mut();
  //   dbg!(a.current_block());
  //   // dbg!(a)
  // }
}
