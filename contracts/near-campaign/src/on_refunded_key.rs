use crate::*;
use near_sdk::is_promise_success;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn on_refunded_key(&mut self, public_key: PublicKey) -> bool {
    let refund_succeeded = is_promise_success();
    env::log_str(format!("The link is refunded: {}", refund_succeeded).as_str());
    if refund_succeeded {
      self.keys.insert(&public_key, &KeyStatus::Refunded);
      self.keys_stats.active -= 1;
      self.keys_stats.refunded += 1;

      if self.keys_stats.active == 0 {
        self.status = CampaignStatus::Completed;
      };

      Promise::new(env::current_account_id()).delete_key(public_key);
    }
    refund_succeeded
  }
}
