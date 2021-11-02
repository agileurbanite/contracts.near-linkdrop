use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[init]
  pub fn new() -> Self {
    Self {
      drops: LookupMap::new(b"d"),
    }
  }
}
