use super::utils::{create_campaign, get_context, keys};
use crate::*;
use crate::clear_state::ClearStatus;
use near_sdk::testing_env;

#[test]
fn clear_state_successful() {
  let keys = keys::get_public_keys(0, 99);
  testing_env!(get_context());

  let mut contract = create_campaign();
  // It is planned to add 100 keys
  contract.keys_stats.total = 100;
  contract.add_keys(keys.clone());
  assert_eq!(100, contract.keys.len());
  assert_eq!(CampaignStatus::Active, contract.status);

  match contract.clear_state(keys.clone()) {
    ClearStatus::Completed(status) => assert_eq!(status, true),
  };
  assert_eq!(0, contract.keys.len());
  assert_eq!(100, contract.keys_stats.deleted_during_deletion);
  assert_eq!(CampaignStatus::Deletion, contract.status);
}
