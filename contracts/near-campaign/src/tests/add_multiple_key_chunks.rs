use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
fn add_multiple_key_chunks() {
  testing_env!(get_context());
  let mut contract = create_campaign();
  // It is planned to add 100 keys
  contract.keys_stats.total = 100;

  let first_chunk = keys::get_public_keys(0, 49);
  let first_chunk_last_key = first_chunk[49].clone();

  contract.add_keys(first_chunk);

  assert_eq!(
    Some(KeyStatus::Active),
    contract.keys.get(&first_chunk_last_key.into())
  );
  assert_eq!(50, contract.keys.len());
  assert_eq!(50, contract.keys_stats.active);
  // The company does not yet have an active status
  assert_eq!(CampaignStatus::Creation, contract.status);

  let second_chunk = keys::get_public_keys(50, 99);
  let second_chunk_last_key = second_chunk[49].clone();

  contract.add_keys(second_chunk);

  assert_eq!(
    Some(KeyStatus::Active),
    contract.keys.get(&second_chunk_last_key.into())
  );
  assert_eq!(100, contract.keys.len());
  assert_eq!(100, contract.keys_stats.active);
  assert_eq!(100, contract.keys_stats.added_during_creation);
  // All keys added, company status must be 'Active'
  assert_eq!(CampaignStatus::Active, contract.status);
}
