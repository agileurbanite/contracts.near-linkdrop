mod create_nft_campaign;

use common::tgas;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::json;
use near_sdk::{env, is_promise_success, near_bindgen, AccountId, Balance, Promise, PublicKey};

const NFT_CAMPAIGN: &[u8] = include_bytes!("../../../../wasm/nft_campaign.wasm");

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NftCampaignCreator {}
