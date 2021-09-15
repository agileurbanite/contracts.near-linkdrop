use crate::*;
use near_sdk::is_promise_success;

#[near_bindgen]
impl User {
  pub fn on_near_campaign_deleted(&mut self) -> bool {
    let campaign_id = env::predecessor_account_id();

    assert!(
      self.campaigns.contains(&campaign_id),
      "Only a user campaign can call this callback"
    );

    let is_campaign_deleted = is_promise_success();
    env::log_str(format!("Is campaign deleted: {}", is_campaign_deleted).as_str());

    if is_campaign_deleted {
      self.campaigns.remove(&campaign_id);
      true
    } else {
      false
    }
  }
}
