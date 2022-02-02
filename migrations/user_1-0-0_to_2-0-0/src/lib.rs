use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OldState {
  future_campaign_id: u64,
  campaigns: UnorderedSet<AccountId>,
  version: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct User {
  campaigns: UnorderedSet<AccountId>,
  version: String,
}

#[near_bindgen]
impl User {
  #[private]
  #[init(ignore_state)]
  pub fn migrate() -> Self {
    let old_state: OldState =
      env::state_read().expect("Migration failed. Can't read contract state");

    Self {
      campaigns: old_state.campaigns,
      version: "2.0.0".to_string(),
    }
  }

  pub fn get_user_metadata(self) -> String {
    "Migration from 1.0.0 to 2.0.0 in progress".to_string()
  }
}
