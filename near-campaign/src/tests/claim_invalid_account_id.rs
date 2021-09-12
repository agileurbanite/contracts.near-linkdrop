use super::utils::{create_campaign, keys, get_context};
use near_sdk::{testing_env, AccountId};

#[test]
#[should_panic]
fn claim_invalid_account_id() {
  let keys = keys::get_public_keys(0, 0);
  let mut context = get_context();

  context.signer_account_id = "b.testnet".parse().unwrap();
  context.predecessor_account_id = "b.testnet".parse().unwrap();
  context.signer_account_pk = keys[0].clone().into();
  context.account_balance = 1_000_000_000_000_000_000_000_000;

  testing_env!(context);

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());

  contract.claim(AccountId::new_unchecked("c*testnet".to_string()));
}
