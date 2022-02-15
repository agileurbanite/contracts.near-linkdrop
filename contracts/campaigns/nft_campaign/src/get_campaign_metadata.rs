use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub campaign_type: String,
  pub created_at: u64,
  pub version: String,
}

#[near_bindgen]
impl NftCampaign {
  pub fn get_campaign_metadata(self) -> Metadata {
    Metadata {
      campaign_type: self.campaign_type,
      created_at: self.created_at,
      version: self.version,
    }
  }
}
