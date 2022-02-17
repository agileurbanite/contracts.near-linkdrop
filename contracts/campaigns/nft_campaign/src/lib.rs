use crate::helpers::{
  log_failed_nft_transfer, log_successful_nft_transfer, nft_transfer, update_drop_status,
};
use common::tgas;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{
  env, is_promise_success, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey,
};

mod cancel_drop;
mod claim;
mod get_campaign_metadata;
mod get_drops;
mod helpers;
mod new;
mod nft_on_transfer;

pub type TokenId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
  pub token_id: TokenId,
  pub collection_id: AccountId,
  pub previous_owner_id: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum DropStatus {
  Active,
  Claimed,
  Canceled,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Drop {
  pub status: DropStatus,
  pub nft: NFT,
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
pub struct DropsStats {
  pub total: u64,
  pub added_during_creation: u64,
  pub deleted_during_deletion: u64,
  pub active: u64,
  pub claimed: u64,
  pub canceled: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub campaign_type: String,
  pub status: CampaignStatus,
  pub created_at: u64,
  pub version: String,
  pub redirect_url: Option<String>,
  pub drops_stats: DropsStats,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct NftCampaign {
  pub metadata: Metadata,
  pub collections_list: UnorderedSet<AccountId>,
  pub drops: LookupMap<PublicKey, Drop>,
}
