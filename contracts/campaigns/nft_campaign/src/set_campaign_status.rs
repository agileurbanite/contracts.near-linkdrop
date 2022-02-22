use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn set_campaign_status(&mut self, status: CampaignStatus) {
    self.metadata.status = status;
  }
}
