use crate::*;

// TODO add total keys are - we need to create a new campaign with a predefined amount of keys
// instead of calculated in dynamically during add_keys

#[near_bindgen]
impl Campaign {
  #[init]
  pub fn new(campaign_id: u64, tokens_per_key: U128, account_creator: String) -> Self {
    Self {
      campaign_id,
      tokens_per_key: tokens_per_key.into(),
      created_at: env::block_timestamp(),
      account_creator: AccountId::new_unchecked(account_creator),
      keys_stats: KeysStats {
        total: 0,
        active: 0,
        created: 0,
        claimed: 0,
        refunded: 0,
      },
      keys: UnorderedMap::new(b"k"),
      version: "1.0".to_string(),
    }
  }
}
