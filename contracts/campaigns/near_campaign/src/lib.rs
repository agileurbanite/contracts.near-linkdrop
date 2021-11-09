use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{
  env,
  ext_contract,
  near_bindgen,
  AccountId,
  Balance,
  PanicOnDefault,
  Promise,
  PublicKey
};

mod add_keys;
mod claim;
pub mod clear_state;
mod create_account_and_claim;
mod delete_campaign;
mod gas;
pub mod get_campaign_metadata;
mod get_key_balance;
pub mod get_keys;
mod new;
mod on_account_created_and_claimed;
mod on_claimed;
mod on_refunded_key;
mod refund_keys;

#[cfg(test)]
mod tests;

#[ext_contract(ext_self)]
pub trait ExtSelfCampaign {
  // Callback after claiming link
  fn on_claimed(&mut self) -> bool;

  // Callback after creating account and claiming link
  fn on_account_created_and_claimed(&mut self) -> bool;

  // Callback after refunded the key
  fn on_refunded_key(&mut self, public_key: PublicKey) -> bool;
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum CampaignStatus {
  Creation,
  Active,
  Completed,
  Deletion,
}

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
  pub total: u64,
  pub added_during_creation: u64,
  pub deleted_during_deletion: u64,
  pub active: u64,
  pub created: u64,
  pub claimed: u64,
  pub refunded: u64,
}

/*
   `campaign_id` - internal id for correct key generation on the frontend.
   `user_id` - linkdrop user account id who created this campaign.
   `account_creator` - account of the original linkdrop (root account) 'testnet' in testnet
    and 'near' in mainnet.
*/

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Campaign {
  campaign_id: u64,
  user_id: AccountId,
  tokens_per_key: Balance,
  created_at: u64,
  account_creator: AccountId,
  keys_stats: KeysStats,
  keys: UnorderedMap<PublicKey, KeyStatus>,
  status: CampaignStatus,
  version: String,
}
