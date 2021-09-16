use crate::*;
use near_sdk::{ext_contract, is_promise_success, Gas};
use std::cell::RefCell;
use std::rc::Rc;

#[ext_contract]
pub trait ExtSelf {
  fn on_added_key(&self, key: PublicKey);
}

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn add_keys_test(&mut self, keys: Vec<PublicKey>) {
    assert_eq!(
      self.status,
      CampaignStatus::Creation,
      "Unable to call add_keys after creating a campaign"
    );

    let mut promise = Rc::new(RefCell::new(Promise::new(env::current_account_id())));

    keys.into_iter().for_each(|key| {
      let add_key_promise = Promise::new(env::current_account_id())
        .add_access_key(
          key.clone(),
          1_000_000_000_000_000_000_000_000,
          env::current_account_id(),
          "create_account_and_claim,claim".to_string(),
        )
        .then(ext_self::on_added_key(
          key,
          env::current_account_id(),
          0,
          Gas(10_000_000_000_000),
        ));

      promise.borrow().and(add_key_promise);
    });

    // promise.into_inner().as_return()
  }

  pub fn on_added_key(&mut self, key: PublicKey) -> bool {
    let is_key_added = is_promise_success();

    env::log_str(format!("Is key added: {}", is_key_added).as_str());

    if is_key_added {
      self.keys.insert(&key, &KeyStatus::Active);
      self.keys_stats.added_during_creation += 1;
      self.keys_stats.active += 1;
      true
    } else {
      false
    }
  }
}
