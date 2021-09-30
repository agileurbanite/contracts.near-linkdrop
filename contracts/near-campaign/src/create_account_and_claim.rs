use crate::gas::*;
use crate::*;

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
    assert_eq!(
      self.keys.get(&env::signer_account_pk()),
      Some(KeyStatus::Active),
      "Cannot create account by inactive or non-existing key"
    );

    Promise::new(self.account_creator.clone())
      .function_call(
        "create_account".to_string(),
        json!({ "new_account_id": new_account_id, "new_public_key": new_public_key })
          .to_string()
          .into_bytes(),
        self.tokens_per_key,
        t_gas(50), // 50 TGas
      )
      .then(ext_self::on_account_created_and_claimed(
        env::current_account_id(),
        0,
        BASE_GAS, // 25 TGas
      ))
  }
}
