use crate::*;
use near_sdk::{AccountId, Gas, PublicKey};

const BASE_GAS: Gas = Gas(25_000_000_000_000); // 25 TGas

#[near_bindgen]
impl Linkdrop {
  #[payable]
  pub fn create_user_account(name: String, public_key: PublicKey) -> Promise {
    let account_id = AccountId::new_unchecked(format!("{}.{}", name, env::current_account_id()));

    Promise::new(account_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(USER_WASM.to_vec())
      .function_call(
        "new".to_string(),
        b"{}".to_vec(),
        0,
        BASE_GAS,
      )
      .then(ext_self::on_user_created(
        U128::from(env::attached_deposit()),
        env::predecessor_account_id(),
        env::current_account_id(),
        0,
        BASE_GAS,
      ))
  }
}
