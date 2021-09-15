use super::utils::{create_campaign, get_context, keys};
use crate::KeyStatus;
use near_sdk::testing_env;

#[test]
fn refund_two_keys() {
  let keys = keys::get_public_keys(0, 1);
  let mut context = get_context();
  context.account_balance = 2_000_000_000_000_000_000_000_000;
  testing_env!(context);

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());

  contract.refund_keys(keys.clone(), "b.testnet".parse().unwrap());

  assert_eq!(
    Some(KeyStatus::Refunded),
    contract.keys.get(&keys[0].clone().into())
  );
  assert_eq!(
    Some(KeyStatus::Refunded),
    contract.keys.get(&keys[1].clone().into())
  );
  assert_eq!(2, contract.keys.len());
  assert_eq!(2, contract.keys_stats.total);
  assert_eq!(2, contract.keys_stats.refunded);
  assert_eq!(0, contract.keys_stats.active);
}
