use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};

mod create_near_campaign;
mod new;

// TODO Try to download contract code from linkdrop contract instead of embed it into the user contract
const NEAR_CAMPAIGN_WASM: &[u8] = include_bytes!("../../wasm/near_campaign.wasm");

setup_alloc!();

#[near_bindgen]
pub struct User {}
