use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  tokens_per_key: U128,
  keys_stats: KeysStats,
}

#[near_bindgen]
impl Campaign {
  pub fn get_campaign(&self) -> Metadata {
    Metadata {
      tokens_per_key: self.tokens_per_key.into(),
      keys_stats: self.keys_stats,
    }
  }
}
