use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise};

mod create_near_campaign;
mod get_campaigns;
mod new;
mod on_near_campaign_created;
mod on_near_campaign_deleted;

// TODO Try to download contract code from linkdrop contract instead of embed it into the user contract
const NEAR_CAMPAIGN_WASM: &[u8] = include_bytes!("../../wasm/near_campaign.wasm");

#[ext_contract(ext_self_user)]
pub trait ExtSelfUser {
  fn on_near_campaign_created(&mut self, campaign_id: AccountId) -> bool;
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct User {
  future_campaign_id: u64,
  campaigns: UnorderedSet<AccountId>,
}
