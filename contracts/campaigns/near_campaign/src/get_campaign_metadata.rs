use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub campaign_id: u64,
  pub user_id: AccountId,
  pub tokens_per_key: U128,
  pub created_at: u64,
  pub account_creator: AccountId,
  pub keys_stats: KeysStats,
  pub status: CampaignStatus,
  pub version: String,
}

#[near_bindgen]
impl Campaign {
  pub fn get_campaign_metadata(self) -> Metadata {
    Metadata {
      campaign_id: self.campaign_id,
      user_id: self.user_id,
      tokens_per_key: self.tokens_per_key.into(),
      created_at: self.created_at,
      account_creator: self.account_creator,
      keys_stats: self.keys_stats,
      status: self.status,
      version: self.version,
    }
  }
}
