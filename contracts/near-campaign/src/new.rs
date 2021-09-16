use crate::*;

#[near_bindgen]
impl Campaign {
  #[init]
  pub fn new(
    campaign_id: u64,
    total_keys: u64,
    tokens_per_key: U128,
    account_creator: AccountId,
    user_id: AccountId,
  ) -> Self {
    Self {
      campaign_id,
      user_id,
      tokens_per_key: tokens_per_key.into(),
      created_at: env::block_timestamp(),
      account_creator,
      keys_stats: KeysStats {
        total: total_keys,
        added_during_creation: 0,
        deleted_during_deletion: 0,
        active: 0,
        created: 0,
        claimed: 0,
        refunded: 0,
      },
      keys: UnorderedMap::new(b"k"),
      status: CampaignStatus::Creation,
      version: "1.0".to_string(),
    }
  }
}
