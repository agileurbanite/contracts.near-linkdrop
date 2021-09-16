use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn refund_keys(&mut self, keys: Vec<PublicKey>, beneficiary_id: AccountId) {
    keys.iter().for_each(|pk| {
      let key = pk.clone().into();

      self.keys.insert(&key, &KeyStatus::Refunded);
      self.keys_stats.active -= 1;
      self.keys_stats.refunded += 1;

      if self.keys_stats.active == 0 {
        self.status = CampaignStatus::Completed;
      };

      Promise::new(env::current_account_id())
        .delete_key(key)
        .then(Promise::new(beneficiary_id.clone()).transfer(self.tokens_per_key));
    });
  }
}
