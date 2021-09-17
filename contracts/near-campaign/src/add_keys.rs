use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn add_keys(&mut self, keys: Vec<PublicKey>) {
    assert_eq!(
      self.status,
      CampaignStatus::Creation,
      "Unable to call this method after creating a campaign"
    );

    keys.into_iter().for_each(|key| {
      assert_eq!(self.keys.get(&key), None, "Key is already exists");

      self.keys.insert(&key, &KeyStatus::Active);
      self.keys_stats.added_during_creation += 1;
      self.keys_stats.active += 1;

      if self.keys_stats.total == self.keys_stats.added_during_creation {
        self.status = CampaignStatus::Active;
      }

      Promise::new(env::current_account_id()).add_access_key(
        key,
        1_000_000_000_000_000_000_000_000, // 1 NEAR
        env::current_account_id(),
        "create_account_and_claim,claim".to_string(),
      );
    });
  }
}
