use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[init]
  pub fn new(
    total: u64,
    collections_whitelist: Vec<AccountId>,
    redirect_url: Option<String>,
  ) -> Self {
    let mut collections_set = UnorderedSet::new(b"c");

    collections_whitelist.into_iter().for_each(|collection_id| {
      collections_set.insert(&collection_id);
    });

    Self {
      metadata: Metadata {
        campaign_type: "nft".to_string(),
        status: CampaignStatus::Initialized,
        created_at: env::block_timestamp(),
        version: "1.0.0".to_string(),
        redirect_url,
        drops_stats: DropsStats {
          total,
          active: 0,
          claimed: 0,
          canceled: 0,
          removed: 0,
        },
      },
      collections_whitelist: collections_set,
      drops: UnorderedMap::new(b"d"),
    }
  }
}
