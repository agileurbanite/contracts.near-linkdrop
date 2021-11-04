use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey};

mod claim;
mod get_drops;
mod new;
mod nft_on_transfer;
mod utils;

pub type TokenId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
  pub token_id: TokenId,
  pub contract_id: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum DropStatus {
  ACTIVE,
  CLAIMED,
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
  pub drops: LookupMap<PublicKey, Drop>,
}
