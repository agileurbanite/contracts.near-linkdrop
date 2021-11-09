use crate::*;
use near_sdk::is_promise_success;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn on_claimed(&mut self) -> bool {
    let claim_succeeded = is_promise_success();
    env::log_str(format!("The link is claimed: {}", claim_succeeded).as_str());
    if claim_succeeded {
      let key = env::signer_account_pk();
      self.keys.insert(&key, &KeyStatus::Claimed);
      self.keys_stats.active -= 1;
      self.keys_stats.claimed += 1;

      if self.keys_stats.active == 0 {
        self.status = CampaignStatus::Completed;
      };

      Promise::new(env::current_account_id()).delete_key(key);
    }
    claim_succeeded
  }
}
