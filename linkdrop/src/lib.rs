use near_sdk::json_types::Base58PublicKey;
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

mod create_user_account;

const USER_WASM: &[u8] = include_bytes!("../../wasm/user.wasm");

setup_alloc!();

#[near_bindgen]
pub struct Linkdrop {}
