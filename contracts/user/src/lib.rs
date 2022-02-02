mod add_campaign_to_list;
mod get_campaigns;
mod get_user_metadata;
mod new;
mod on_near_campaign_deleted;
mod remove_campaign_from_list;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, is_promise_success, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct User {
  campaigns: UnorderedSet<AccountId>,
  version: String,
}
