use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
fn add_one_key() {
  testing_env!(get_context());
  let mut contract = create_campaign();
  let keys = keys::get_public_keys(0, 0);

  contract.add_keys(keys.clone());

  assert_eq!(
    Some(KeyStatus::Active),
    contract.keys.get(&keys[0].clone().into())
  );
  assert_eq!(1, contract.keys.len());
  assert_eq!(1, contract.keys_stats.active);
  assert_eq!(1, contract.keys_stats.added_during_creation);
  assert_eq!(CampaignStatus::Active, contract.status);
}
