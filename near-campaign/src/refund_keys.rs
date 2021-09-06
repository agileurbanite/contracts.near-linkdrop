use crate::*;
use near_sdk::json_types::ValidAccountId;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn refund_keys(&mut self, keys: Vec<Base58PublicKey>, beneficiary_id: ValidAccountId) {
    keys.iter().for_each(|pk| {
      let key = pk.clone().into();

      self.keys.insert(&key, &KeyStatus::Refunded);
      self.keys_stats.active -= 1;
      self.keys_stats.refunded += 1;

      Promise::new(env::current_account_id())
        .delete_key(key)
        .then(Promise::new(beneficiary_id.to_string()).transfer(self.tokens_per_key));
    });
  }
}
