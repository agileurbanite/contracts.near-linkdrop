use crate::*;
use near_sdk::PromiseOrValue;

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn remove_drop(&mut self, key: PublicKey, receiver_id: AccountId) -> PromiseOrValue<bool> {
    let drop = self
      .drops
      .get(&key)
      .expect("Cannot delete non-existing drop");

    if drop.status != DropStatus::Active {
      self.drops.remove(&key);
      self.metadata.drops_stats.removed += 1;
      return PromiseOrValue::Value(true);
    }

    let promise = nft_transfer(drop.nft.collection_id, drop.nft.token_id, receiver_id).then(
      Promise::new(env::current_account_id()).function_call(
        "on_delete_drop".to_string(),
        json!({ "key": key }).to_string().into_bytes(),
        0,
        tgas(20),
      ),
    );
    PromiseOrValue::Promise(promise)
  }

  #[private]
  pub fn on_delete_drop(&mut self, key: PublicKey) -> bool {
    if !is_promise_success() {
      return false;
    }

    self.drops.remove(&key);
    self.metadata.drops_stats.removed += 1;
    Promise::new(env::current_account_id()).delete_key(key);
    true
  }
}
