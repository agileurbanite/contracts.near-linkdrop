use near_sdk::{Gas, PublicKey};

use crate::*;

// TODO Add last_campaign_id (1,2,3...) - need to use with path for generating keys
// TODO Validate attached amount of NEAR, add total_keys field
#[near_bindgen]
impl User {
  #[payable]
  #[private]
  pub fn create_near_campaign(
    name: AccountId,
    public_key: PublicKey,
    tokens_per_key: U128,
  ) -> Promise {
    let campaign_id = AccountId::new_unchecked(format!("{}.{}", name, env::current_account_id()));

    Promise::new(campaign_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(NEAR_CAMPAIGN_WASM.to_vec())
      .function_call(
        "new".to_string(),
        json!({ "tokens_per_key": tokens_per_key })
          .to_string()
          .into_bytes(),
        0,
        Gas(50_000_000_000_000),
      )
  }
}
