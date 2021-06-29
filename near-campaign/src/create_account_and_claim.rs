use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn create_account_and_claim(
    &mut self,
    new_account_id: AccountId,
    new_public_key: Base58PublicKey,
  ) -> Promise {
    let key = env::signer_account_pk();

    self.keys.insert(&key, &KeyStatus::Created);
    self.keys_stats.active -= 1;
    self.keys_stats.created += 1;

    Promise::new(EXTERNAL_LINKDROP_ACCOUNT.to_string())
      .function_call(
        b"create_account".to_vec(),
        json!({ "new_account_id": new_account_id, "new_public_key": new_public_key })
          .to_string()
          .as_bytes()
          .to_vec(),
        self.tokens_per_key,
        50_000_000_000_000, // 50 Tgas
      )
      .then(Promise::new(env::current_account_id()).delete_key(key))
  }
}
