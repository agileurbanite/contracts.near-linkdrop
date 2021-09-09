use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};

mod create_user_account;

const USER_WASM: &[u8] = include_bytes!("../../wasm/user.wasm");

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Linkdrop {}
