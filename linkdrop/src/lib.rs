use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58PublicKey};
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

const USER_WASM: &[u8] = include_bytes!("../../wasm/user.wasm");

setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Linkdrop {}

// TODO instead of getting a name from the args get it from signer_account_id (as we do it on the UI)
#[near_bindgen]
impl Linkdrop {
  #[payable]
  pub fn create_user_account(&mut self, name: String, public_key: Base58PublicKey) -> Promise {
    let account_id = format!("{}.{}", name, env::current_account_id());

    Promise::new(account_id)
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(USER_WASM.to_vec())
  }
}
