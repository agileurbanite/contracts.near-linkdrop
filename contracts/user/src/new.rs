use crate::*;

#[near_bindgen]
impl User {
  #[init]
  pub fn new() -> Self {
    Self {
      future_campaign_id: 1,
      campaigns: UnorderedSet::new(b"c"),
      version: "1.0".to_string(),
    }
  }
}
