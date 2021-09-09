use crate::*;
use near_sdk::is_promise_success;

#[near_bindgen]
impl User {
  #[private]
  pub fn on_near_campaign_created(&mut self) -> bool {
    let is_campaign_created = is_promise_success();
    env::log_str(format!("Is campaign created: {}", is_campaign_created).as_str());

    if is_campaign_created {
      self.future_campaign_id += 1;
      true
    } else {
      false
    }
  }
}
