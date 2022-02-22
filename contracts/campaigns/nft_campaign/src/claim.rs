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
    let key = env::signer_account_pk();
    let drop = self.drops.get(&key).unwrap();

    if !is_promise_success() {
      return log_failed_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
    }

    self.metadata.drops_stats.active -= 1;
    self.metadata.drops_stats.claimed += 1;
    self
      .drops
      .insert(&key, &update_drop_status(&drop, DropStatus::Claimed)); // TODO check if possible to mutate status directly

    if self.metadata.drops_stats.active == 0 {
      self.metadata.status = CampaignStatus::Completed;
    };

    log_successful_nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id);
    Promise::new(env::current_account_id()).delete_key(key);
  }
}
