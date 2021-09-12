use super::utils::{create_campaign, keys, get_context};
use crate::*;
use near_sdk::testing_env;

#[test]
fn balance_two_keys() {
  let keys = keys::get_public_keys(0, 1);
  testing_env!(get_context());

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());

  let balance1 = contract.get_key_balance(keys[0].clone());
  assert_eq!(U128::from(1_000_000_000_000_000_000_000_000), balance1);

  let balance2 = contract.get_key_balance(keys[1].clone());
  assert_eq!(U128::from(1_000_000_000_000_000_000_000_000), balance2);
}
