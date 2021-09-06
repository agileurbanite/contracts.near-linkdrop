use crate::*;
use near_sdk::Gas;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn create_account_and_claim(
    &mut self,
    new_account_id: AccountId,
    new_public_key: PublicKey,
  ) -> Promise {
    let key = env::signer_account_pk();

    self.keys.insert(&key, &KeyStatus::Created);
    self.keys_stats.active -= 1;
    self.keys_stats.created += 1;

    // TODO We need to check if the account was successfully created. Now the key will be deleted
    // even if we will get an error and the account wasn't created.
    Promise::new(AccountId::new_unchecked(
      EXTERNAL_LINKDROP_ACCOUNT.to_string(),
    ))
    .function_call(
      "create_account".to_string(),
      json!({ "new_account_id": new_account_id, "new_public_key": new_public_key })
        .to_string()
        .into_bytes(),
      self.tokens_per_key,
      Gas(50_000_000_000_000), // 50 Tgas
    )
    .then(Promise::new(env::current_account_id()).delete_key(key))
  }
}
