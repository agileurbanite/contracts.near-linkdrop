use crate::gas::*;
use crate::*;
use near_sdk::ext_contract;

#[ext_contract]
pub trait ExtUser {
  fn on_near_campaign_deleted(&self);
}

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn delete_campaign(self, beneficiary_id: AccountId) -> Promise {
    Promise::new(env::current_account_id())
      .delete_account(beneficiary_id.clone())
      .then(ext_user::on_near_campaign_deleted(
        self.user_id,
        0,
        BASE_GAS, // 25 TGas
      ))
  }
}
