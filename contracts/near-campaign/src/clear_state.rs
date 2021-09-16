use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ClearStatus {
  Completed(bool),
}

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn clear_state(&mut self, keys: Vec<PublicKey>) -> ClearStatus {
    self.status = CampaignStatus::Deletion;

    keys.into_iter().for_each(|pk| {
      let key = pk.into();
      self.keys_stats.deleted_during_deletion += 1;
      let key_status = self.keys.remove(&key);

      if key_status == Some(KeyStatus::Active) {
        Promise::new(env::current_account_id()).delete_key(key);
      }
    });

    ClearStatus::Completed(self.keys.is_empty())
  }
}
