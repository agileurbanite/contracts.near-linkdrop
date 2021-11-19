use near_sdk::AccountId;
use near_sdk_sim::{to_yocto, UserAccount};
use std::rc::Rc;

pub struct Person {
  pub account: UserAccount,
}

impl Person {
  pub fn new(root_account: Rc<UserAccount>, account_id: &str, init_balance: &str) -> Self {
    let account = root_account.create_user(
      AccountId::new_unchecked(account_id.to_string()),
      to_yocto(init_balance),
    );
    Person { account }
  }

  pub fn create_alice(root_account: Rc<UserAccount>, init_balance: &str) -> Self {
    Self::new(root_account, "alice", init_balance)
  }
}
