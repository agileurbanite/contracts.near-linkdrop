use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"BalanceExceeded"#)]
fn claim_balance_exceeded() {
  let keys = keys::get_public_keys(0, 0);
  let mut context = get_context();

  context.signer_account_id = "b.testnet".parse().unwrap();
  context.predecessor_account_id = "b.testnet".parse().unwrap();
  context.signer_account_pk = keys[0].clone().into();

  testing_env!(context);

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());

  contract.claim("c.testnet".parse().unwrap());
}
