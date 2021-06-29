use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn delete_keys(&mut self, keys: Vec<Base58PublicKey>) {
    keys.iter().for_each(|pk| {
      let key = pk.clone().into();

      self.keys.insert(&key, &KeyStatus::Deleted);
      self.keys_stats.active -= 1;
      self.keys_stats.deleted += 1;

      Promise::new(env::current_account_id()).delete_key(key);
      // TODO: Do we need to transfer tokens back to the main user account (acc, which created the campaign)?
    });
  }
}
