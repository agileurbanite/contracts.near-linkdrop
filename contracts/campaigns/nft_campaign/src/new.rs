use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[init]
  pub fn new(total_drops: u64, redirect_url: Option<String>) -> Self {
    Self {
      metadata: Metadata {
        campaign_type: "nft".to_string(),
        status: CampaignStatus::Creation,
        created_at: env::block_timestamp(),
        version: "1.0.0".to_string(),
        redirect_url,
        drops_stats: DropsStats {
          total: total_drops,
          added_during_creation: 0,
          deleted_during_deletion: 0,
          active: 0,
          claimed: 0,
          canceled: 0,
        },
      },
      collections_list: UnorderedSet::new(b"c"),
      drops: LookupMap::new(b"d"),
    }
  }
}
