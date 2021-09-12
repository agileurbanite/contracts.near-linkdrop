use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub campaign_id: u64,
  pub tokens_per_key: U128,
  pub created_at: u64,
  pub account_creator: AccountId,
  pub keys_stats: KeysStats,
  pub status: String,
  pub version: String,
}

#[near_bindgen]
impl Campaign {
  pub fn get_campaign_metadata(self) -> Metadata {
    // TODO rework it! keys.len() is a wrong param - use keys_stats.active instead
    let status = if self.keys.len() > 0 {
      "active"
    } else {
      "completed"
    };

    Metadata {
      campaign_id: self.campaign_id,
      tokens_per_key: self.tokens_per_key.into(),
      created_at: self.created_at,
      account_creator: self.account_creator,
      keys_stats: self.keys_stats,
      status: status.to_string(),
      version: self.version,
    }
  }
}
