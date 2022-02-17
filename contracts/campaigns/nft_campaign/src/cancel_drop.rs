use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn cancel_drop(&mut self, key: PublicKey, receiver_id: AccountId) -> Promise {
    let drop = self
      .drops
      .get(&key)
      .expect("Cannot cancel non-existing drop");

    assert_eq!(
      drop.status,
      DropStatus::Active,
      "Cannot cancel inactive drop"
    );

    nft_transfer(
      drop.nft.collection_id,
      drop.nft.token_id,
      receiver_id.clone(),
    )
    .then(
      Promise::new(env::current_account_id()).function_call(
        "on_cancel_drop".to_string(),
        json!({
          "key": key,
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
  pub fn on_cancel_drop(&mut self, key: PublicKey, receiver_id: AccountId) {
    let drop = self.drops.get(&key).unwrap();

    if is_promise_success() {
      self
        .drops
        .insert(&key, &update_drop_status(&drop, DropStatus::Canceled));

      log_successful_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
      return;
    }

    log_failed_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
  }
}
