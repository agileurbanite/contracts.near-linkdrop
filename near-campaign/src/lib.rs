use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{
  env, ext_contract, near_bindgen, setup_alloc, AccountId, Balance, PanicOnDefault, Promise,
  PublicKey,
};

mod add_keys;
mod claim;
mod create_account_and_claim;
mod delete_keys;
mod get_campaign_metadata;
mod get_key_balance;
mod get_keys;
mod new;

setup_alloc!();

// TODO should be testnet or near - depends of the network
const EXTERNAL_LINKDROP_ACCOUNT: &str = "testnet";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum KeyStatus {
  Active,
  Created,
  Claimed,
  Deleted,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct KeysStats {
  total: u64,
  active: u64,
  created: u64,
  claimed: u64,
  deleted: u64,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Campaign {
  tokens_per_key: Balance,
  keys_stats: KeysStats,
  created_at: u64,
  keys: UnorderedMap<PublicKey, KeyStatus>,
}
