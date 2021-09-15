use crate::*;
use near_sdk::{Gas, PublicKey};

// TODO Validate attached amount of NEAR, add total_keys field
#[near_bindgen]
impl User {
  #[payable]
  #[private]
  pub fn create_near_campaign(
    &mut self,
    name: String, // TODO Need to validate the name. NO '.', e.g 'my.campaign' has to be invalid
    public_key: PublicKey,
    tokens_per_key: U128,
    account_creator: AccountId,
  ) -> Promise {
    let campaign_id = AccountId::new_unchecked(format!("{}.{}", name, env::current_account_id()));

    Promise::new(campaign_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(NEAR_CAMPAIGN_WASM.to_vec())
      .function_call(
        "new".to_string(),
        json!({
          "campaign_id": self.future_campaign_id,
          "tokens_per_key": tokens_per_key,
          "account_creator": account_creator,
          "user_id": env::current_account_id()
        })
        .to_string()
        .into_bytes(),
        0,
        Gas(50_000_000_000_000),
      )
      .then(ext_self_user::on_near_campaign_created(
        campaign_id,
        env::current_account_id(),
        0,
        Gas(25_000_000_000_000),
      ))
  }
}
