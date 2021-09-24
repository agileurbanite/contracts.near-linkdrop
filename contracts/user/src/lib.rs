use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise};

mod create_near_campaign;
mod get_campaigns;
mod get_user_metadata;
mod new;
mod on_near_campaign_created;
mod on_near_campaign_deleted;

const NEAR_CAMPAIGN_WASM: &[u8] = include_bytes!("../../../wasm/near_campaign.wasm");

#[cfg(test)]
mod tests;

#[ext_contract(ext_self_user)]
pub trait ExtSelfUser {
  fn on_near_campaign_created(&mut self, campaign_id: AccountId) -> bool;
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct User {
  future_campaign_id: u64,
  campaigns: UnorderedSet<AccountId>,
  version: String,
}
