use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn add_keys(&mut self, keys: Vec<Base58PublicKey>) {
    self.keys_stats.total += keys.len() as u64;

    keys.iter().for_each(|pk| {
      // TODO do we need to check if key is already added to the state?
      let key = pk.clone().into();

      // TODO: use callback for this?
      self.keys.insert(&key, &KeyStatus::Active);
      self.keys_stats.active += 1;

      Promise::new(env::current_account_id()).add_access_key(
        key,
        1_000_000_000_000_000_000_000_000, // 1 NEAR TODO Should we use an unlimited amount?
        env::current_account_id(),
        b"create_account_and_claim,claim".to_vec(),
      );
    });
  }
}
