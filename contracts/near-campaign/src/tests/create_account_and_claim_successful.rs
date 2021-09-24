use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
fn create_account_and_claim_successful() {
  let keys = keys::get_public_keys(0, 0);
  let new_keys = keys::get_public_keys(1, 1);
  let mut context = get_context();

  context.signer_account_id = "b.testnet".parse().unwrap();
  context.predecessor_account_id = "b.testnet".parse().unwrap();
  context.signer_account_pk = keys[0].clone().into();
  context.account_balance = 1_000_000_000_000_000_000_000_000;

  testing_env!(context);

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());
  assert_eq!(CampaignStatus::Active, contract.status);

  contract.create_account_and_claim(
    "c.testnet".parse().unwrap(),
    new_keys[0].clone()
  );

  assert_eq!(
    Some(KeyStatus::Created),
    contract.keys.get(&keys[0].clone().into())
  );
  assert_eq!(1, contract.keys.len());
  assert_eq!(1, contract.keys_stats.created);
  assert_eq!(0, contract.keys_stats.active);
  assert_eq!(CampaignStatus::Completed, contract.status)
}
