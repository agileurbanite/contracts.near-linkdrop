use crate::*;

#[near_bindgen]
impl Campaign {
  // TODO probably we should replace this Vec with some another (for efficient usage)?
  // TODO is it possible that some keys will not be added to the account? Need to check it
  #[private]
  pub fn add_keys(&mut self, keys: Vec<Base58PublicKey>) {
    self.keys_stats.total += keys.len() as u64;

    keys.iter().for_each(|pk| {
      // TODO check if key is already added to the state, if yes - panic
      // TODO Maybe we can improve it and don't clone pk
      let key = pk.clone().into();

      // TODO: use callback for this ??
      self.keys.insert(&key, &KeyStatus::Active);
      self.keys_stats.active += 1;

      // TODO: What is cheaper: add new key or check if the public key exists in the state?
      // Do we attach a new key for paying gas fee from the campaign account?
      Promise::new(env::current_account_id()).add_access_key(
        key,
        1_000_000_000_000_000_000_000_000, // 1 NEAR
        env::current_account_id(),
        b"create_account_and_claim,claim".to_vec(),
      );
    });
  }
}