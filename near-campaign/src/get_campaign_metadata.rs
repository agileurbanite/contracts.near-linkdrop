use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  tokens_per_key: U128,
  keys_stats: KeysStats,
  created_at: u64,
  status: String,
}

#[near_bindgen]
impl Campaign {
  pub fn get_campaign_metadata(&self) -> Metadata {
    // TODO rework it! keys.len() is a wrong param - use keys_stats.active instead
    let status = if self.keys.len() > 0 { "active" } else { "completed" };

    Metadata {
      tokens_per_key: self.tokens_per_key.into(),
      keys_stats: self.keys_stats,
      created_at: self.created_at,
      status: status.to_string(),
    }
  }
}
