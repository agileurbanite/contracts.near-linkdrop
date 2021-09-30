use crate::*;
use near_sdk::is_promise_success;

#[near_bindgen]
impl Linkdrop {
  #[private]
  pub fn on_user_created(
    &mut self,
    attached_deposit: U128,
    payer_account_id: AccountId
  ) -> bool {
    let is_user_created = is_promise_success();
    env::log_str(format!("Is user created: {}", is_user_created).as_str());
    if !is_user_created {
      Promise::new(payer_account_id).transfer(attached_deposit.0);
    }
    is_user_created
  }
}
