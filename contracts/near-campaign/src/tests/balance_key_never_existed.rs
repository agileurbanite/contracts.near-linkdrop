use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Public key was used or never existed"#)]
fn balance_key_never_existed() {
  let keys = keys::get_public_keys(0, 0);
  testing_env!(get_context());

  let mut contract = create_campaign();
  contract.add_keys(keys.clone());

  let unknown_key = keys::get_public_keys(1, 1);
  contract.get_key_balance(unknown_key[0].clone());
}
