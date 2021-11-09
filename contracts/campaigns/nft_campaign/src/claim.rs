use crate::utils::update_drop_status;
use crate::*;
use near_sdk::serde_json::json;
use near_sdk::{is_promise_success, Gas};

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn claim(&mut self, beneficiary_id: AccountId) -> Promise {
    let key = env::signer_account_pk();
    let drop = self.drops.get(&key).expect("No drop associated with key");

    assert_eq!(
      drop.status,
      DropStatus::ACTIVE,
      "Cannot claim inactive drop"
    );

    Promise::new(drop.nft.contract_id.clone())
      .function_call(
        "nft_transfer".to_string(),
        json!({
          "receiver_id": beneficiary_id,
          "token_id": drop.nft.token_id,
        })
        .to_string()
        .into_bytes(),
        1,
        Gas::from(25_000_000_000_000),
      )
      .then(
        Promise::new(env::current_account_id()).function_call(
          "on_claim".to_string(),
          json!({
            "beneficiary_id": beneficiary_id,
          })
          .to_string()
          .into_bytes(),
          0,
          Gas::from(25_000_000_000_000),
        ),
      )
  }

  #[private]
  pub fn on_claim(&mut self, beneficiary_id: AccountId) {
    let is_success = is_promise_success();
    let key = env::signer_account_pk();
    let drop = self.drops.get(&key).unwrap();

    if !is_success {
      env::panic_str(
        format!(
          "Failed to transfer token '{}' of @{} to @{}",
          &drop.nft.token_id, &drop.nft.contract_id, beneficiary_id,
        )
        .as_str(),
      );
    }

    self
      .drops
      .insert(&key, &update_drop_status(&drop, DropStatus::CLAIMED));

    env::log_str(
      format!(
        "Successfully transfer token '{}' of @{} to @{}",
        &drop.nft.token_id, &drop.nft.contract_id, beneficiary_id,
      )
      .as_str(),
    );
  }
}
