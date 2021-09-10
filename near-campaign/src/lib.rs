use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PublicKey};

mod add_keys;
mod claim;
mod clear_state;
mod create_account_and_claim;
mod get_campaign_metadata;
mod get_key_balance;
mod get_keys;
mod new;
mod refund_keys;

#[cfg(test)]
mod tests;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum KeyStatus {
  Active,
  Created,
  Claimed,
  Refunded,
}

#[derive(
  BorshSerialize, BorshDeserialize, Serialize, Deserialize, Copy, Clone, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct KeysStats {
  total: u64,
  active: u64,
  created: u64,
  claimed: u64,
  refunded: u64,
}

/*
  account_creator - account of the original linkdrop (root account) 'testnet' in testnet
  and 'near' in mainnet
 */

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Campaign {
  campaign_id: u64,
  tokens_per_key: Balance,
  created_at: u64,
  account_creator: AccountId,
  keys_stats: KeysStats,
  keys: UnorderedMap<PublicKey, KeyStatus>,
  version: String,
}
