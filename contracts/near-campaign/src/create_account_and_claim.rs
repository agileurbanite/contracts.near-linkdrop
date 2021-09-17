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
    assert_eq!(
      self.status,
      CampaignStatus::Active,
      "Unable to call this method on inactive campaign"
    );

    let key = env::signer_account_pk();

    assert_eq!(
      self.keys.get(&key),
      Some(KeyStatus::Active),
      "Cannot create account by inactive or non-existing key"
    );

    self.keys.insert(&key, &KeyStatus::Created);
    self.keys_stats.active -= 1;
    self.keys_stats.created += 1;

    if self.keys_stats.active == 0 {
      self.status = CampaignStatus::Completed;
    };

    Promise::new(self.account_creator.clone())
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
