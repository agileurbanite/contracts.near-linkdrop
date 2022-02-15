use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey};

mod claim;
mod get_campaign_metadata;
mod get_drops;
mod new;
mod nft_on_transfer;
mod utils;

pub type TokenId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
  pub token_id: TokenId,
  pub collection_id: AccountId,
  pub owner_id: AccountId,
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

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct NftCampaign {
  pub campaign_type: String,
  pub drops: LookupMap<PublicKey, Drop>,
  pub created_at: u64,
  pub version: String,
  // TODO add collections_list for validation of who can call nft_on_transfer?
  // TODO add redirect url
  // TODO add status
  // TODO add drop_statistic
}
