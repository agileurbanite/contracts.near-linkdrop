use crate::*;

#[ext_contract(ext_self)]
pub trait ExtUser {
  fn on_create_campaign(&mut self, account_id: String);
}

// TODO Check if deleting account will delete sub-accounts too
// TODO Add last_campaign_id (1,2,3...) - need to use with path for generating keys
#[near_bindgen]
impl User {
  // TODO Add private mode
  #[payable]
  pub fn create_near_campaign(
    &mut self,
    name: AccountId,
    public_key: Base58PublicKey,
    tokens_per_key: U128,
  ) -> Promise {
    let account_id = format!("{}.{}", name, env::current_account_id());

    Promise::new(account_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(NEAR_CAMPAIGN_WASM.to_vec())
      .function_call(
        b"new".to_vec(),
        json!({ "tokens_per_key": tokens_per_key })
          .to_string()
          .as_bytes()
          .to_vec(),
        0,
        50_000_000_000_000,
      )
      .then(ext_self::on_create_campaign(
        account_id,
        &env::current_account_id(),
        0,
        10_000_000_000_000,
      ))
  }

  #[private]
  pub fn on_create_campaign(&mut self, account_id: String) {
    self.campaigns.push(&account_id);
  }
}
