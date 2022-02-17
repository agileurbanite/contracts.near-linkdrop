use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn claim(&mut self, receiver_id: AccountId) -> Promise {
    let key = env::signer_account_pk();
    let drop = self.drops.get(&key).expect("No drop associated with key");

    assert_eq!(
      drop.status,
      DropStatus::Active,
      "Cannot claim inactive drop"
    );

    nft_transfer(
      drop.nft.collection_id,
      drop.nft.token_id,
      receiver_id.clone(),
    )
    .then(
      Promise::new(env::current_account_id()).function_call(
        "on_claim".to_string(),
        json!({
          "receiver_id": receiver_id,
        })
        .to_string()
        .into_bytes(),
        0,
        tgas(25),
      ),
    )
  }

  #[private]
  pub fn on_claim(&mut self, receiver_id: AccountId) {
    let is_success = is_promise_success();
    let key = env::signer_account_pk();
    let drop = self.drops.get(&key).unwrap();

    if !is_success {
      return log_failed_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
    }

    self
      .drops
      .insert(&key, &update_drop_status(&drop, DropStatus::Claimed));

    log_successful_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
  }
}
