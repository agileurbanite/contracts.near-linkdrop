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
        tgas(20),
      ),
    )
  }

  #[private]
  pub fn on_cancel_drop(&mut self, key: PublicKey, receiver_id: AccountId) {
    let drop = self.drops.get(&key).unwrap();

    if !is_promise_success() {
      return log_failed_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
    }

    self.metadata.drops_stats.active -= 1;
    self.metadata.drops_stats.canceled += 1;
    self
      .drops
      .insert(&key, &update_drop_status(&drop, DropStatus::Canceled));

    if self.metadata.drops_stats.active == 0 {
      self.metadata.status = CampaignStatus::Completed;
    };

    log_successful_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
    Promise::new(env::current_account_id()).delete_key(key);
  }
}
