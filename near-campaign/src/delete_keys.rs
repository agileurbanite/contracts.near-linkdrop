use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn delete_keys(&mut self, keys: Vec<Base58PublicKey>) {
    // Need to fix this approach - what to do if some operation will fail?
    keys.iter().for_each(|pk| {
      let key = pk.clone().into();

      self.keys.insert(&key, &KeyStatus::Deleted);
      self.keys_stats.active -= 1;
      self.keys_stats.deleted += 1;

      // Looks like if we will try to delete non-existing key it won't panic
      Promise::new(env::current_account_id()).delete_key(key);
      // TODO: Do we need to transfer tokens back to the main user account (acc, which created the campaign)?
    });
  }
}
