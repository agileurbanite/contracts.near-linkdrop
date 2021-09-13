use super::utils::{create_campaign, keys, get_context};
use crate::*;
use near_sdk::testing_env;
use crate::get_campaign_metadata::Metadata;

#[test]
fn campaign_metadata() {
  let keys = keys::get_public_keys(0, 9);
  testing_env!(get_context());
  let last_block_timestamp = env::block_timestamp();

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());
  // The 'create_account_and_claim' method was executed for one key
  contract.keys_stats.active -= 1;
  contract.keys_stats.created += 1;

  let metadata: Metadata = contract.get_campaign_metadata().into();

  assert_eq!(U128::from(1_000_000_000_000_000_000_000_000), metadata.tokens_per_key);
  assert!(metadata.created_at >= last_block_timestamp);
  assert_eq!("active", metadata.status);
  assert_eq!(1, metadata.campaign_id);
  assert_eq!(AccountId::new_unchecked("testnet".to_string()), metadata.account_creator);
  assert_eq!("1.0".to_string(), metadata.version);
  assert_eq!(10, metadata.keys_stats.total);
  assert_eq!(9, metadata.keys_stats.active);
  assert_eq!(1, metadata.keys_stats.created);
}
