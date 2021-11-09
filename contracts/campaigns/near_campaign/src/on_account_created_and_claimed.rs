use crate::*;
use near_sdk::{is_promise_success, promise_result_as_success};

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn on_account_created_and_claimed(&mut self) -> bool {
    let mut creation_succeeded = is_promise_success();
    if creation_succeeded {
      let result = String::from_utf8(promise_result_as_success().unwrap());
      if result.is_ok() && result.unwrap() == "false".to_string() {
        creation_succeeded = false;
      }
    }
    env::log_str(format!("The account is created and link is claimed: {}", creation_succeeded).as_str());
    if creation_succeeded {
      let key = env::signer_account_pk();
      self.keys.insert(&key, &KeyStatus::Created);
      self.keys_stats.active -= 1;
      self.keys_stats.created += 1;

      if self.keys_stats.active == 0 {
        self.status = CampaignStatus::Completed;
      };

      Promise::new(env::current_account_id()).delete_key(key);
    }
    creation_succeeded
  }
}
