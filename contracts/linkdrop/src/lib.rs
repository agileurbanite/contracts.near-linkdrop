use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, Promise, AccountId};

mod create_user_account;
mod on_user_created;

const USER_WASM: &[u8] = include_bytes!("../../../wasm/user.wasm");

#[ext_contract(ext_self)]
pub trait ExtSelfLinkdrop {
  fn on_user_created(
    &mut self,
    attached_deposit: U128,
    payer_account_id: AccountId
  ) -> bool;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Linkdrop {}
