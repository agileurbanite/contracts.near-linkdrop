use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[init]
  pub fn new() -> Self {
    Self {
      campaign_type: "nft".to_string(),
      drops: LookupMap::new(b"d"),
      created_at: env::block_timestamp(),
      version: "1.0.0".to_string()
    }
  }
}
