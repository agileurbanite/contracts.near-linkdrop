mod add_keys;
mod claim;
mod create_account_and_claim;
mod delete_keys;
mod get_campaign;
mod get_key_balance;
mod new;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{
  env, near_bindgen, setup_alloc, AccountId, Balance, PanicOnDefault, Promise, PublicKey,
};

setup_alloc!();

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
  keys: UnorderedMap<PublicKey, KeyStatus>,
}
