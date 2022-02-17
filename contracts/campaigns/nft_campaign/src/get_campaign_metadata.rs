use crate::*;

#[near_bindgen]
impl NftCampaign {
  pub fn get_campaign_metadata(self) -> Metadata {
    self.metadata
  }
}
