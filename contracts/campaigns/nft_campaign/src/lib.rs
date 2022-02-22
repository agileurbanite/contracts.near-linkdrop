use crate::helpers::{
  log_failed_nft_transfer, log_successful_nft_transfer, nft_transfer, update_drop_status,
};
use common::tgas;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
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
mod remove_drop;
mod set_campaign_status;

pub type TokenId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
  pub token_id: TokenId,
  pub collection_id: AccountId,
  pub previous_owner_id: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde", rename_all = "camelCase")]
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
#[serde(crate = "near_sdk::serde", rename_all = "camelCase")]
pub enum CampaignStatus {
  Initialized,
  DropsCreation,
  Active,
  Completed,
  DropsDeletion,
}

/*
  canceled - how many drops were canceled by campaign owner
  removed - we need this for tracking drop deletion progress - we need to restore deletion if user
   will interrupt it and will want to continue on another device
 */
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DropsStats {
  pub total: u64,
  pub active: u64,
  pub claimed: u64,
  pub canceled: u64,
  pub removed: u64,
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

/*
 collections list - we use it for validation of nft_on_transfer call - as this method is public,
 and we want to allow to call this method only by whitelisted accounts
*/
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct NftCampaign {
  pub metadata: Metadata,
  pub collections_whitelist: UnorderedSet<AccountId>,
  pub drops: UnorderedMap<PublicKey, Drop>,
}
