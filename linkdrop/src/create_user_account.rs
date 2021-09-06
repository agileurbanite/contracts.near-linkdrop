use crate::*;

#[near_bindgen]
impl Linkdrop {
  #[payable]
  pub fn create_user_account(public_key: Base58PublicKey) -> Promise {
    let account_id = format!("{}.{}", env::signer_account_id(), env::current_account_id());

    Promise::new(account_id)
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(USER_WASM.to_vec())
      .function_call(b"new".to_vec(), b"{}".to_vec(), 0, 20_000_000_000_000)
  }
}
