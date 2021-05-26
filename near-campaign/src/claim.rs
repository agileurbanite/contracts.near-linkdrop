use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn claim(&mut self, account_id: AccountId) -> Promise {
    assert!(
      env::is_valid_account_id(account_id.as_bytes()),
      "Invalid account id"
    );

    let key = env::signer_account_pk();

    // TODO move Claim key feature to separate function
    self.keys.insert(&key, &KeyStatus::Claimed);
    self.keys_stats.active -= 1;
    self.keys_stats.claimed += 1;

    // Maybe don't delete pk from account, just set it claimed?
    Promise::new(env::current_account_id()).delete_key(key);
    Promise::new(account_id).transfer(self.tokens_per_key)
  }
}
