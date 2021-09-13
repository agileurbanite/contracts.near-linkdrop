use super::utils::{create_campaign, keys, get_context};
use near_sdk::testing_env;
use crate::get_keys::Key;
use crate::KeyStatus;

#[test]
fn retrieve_keys() {
  let keys = keys::get_public_keys(0, 9);
  let mut context = get_context();

  context.signer_account_id = "b.testnet".parse().unwrap();
  context.predecessor_account_id = "b.testnet".parse().unwrap();
  context.signer_account_pk = keys[0].clone().into();
  context.account_balance = 1_000_000_000_000_000_000_000_000;

  testing_env!(context);
  let mut contract = create_campaign();
  contract.add_keys(keys.clone());
  contract.claim("c.testnet".parse().unwrap());

  let stored_keys: Vec<Key> = contract.get_keys(keys.clone());
  assert_eq!(10, stored_keys.len());

  assert_eq!(Some(KeyStatus::Claimed), stored_keys[0].status);
  assert_eq!(keys[0], stored_keys[0].pk);

  assert_eq!(Some(KeyStatus::Active), stored_keys[9].status);
  assert_eq!(keys[9], stored_keys[9].pk);
}
