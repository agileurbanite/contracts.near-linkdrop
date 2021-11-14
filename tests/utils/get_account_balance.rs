use crate::utils::Runtime;
use near_sdk_sim::types::Balance;

pub fn get_account_balance(account_id: &str, runtime: &Runtime) -> Balance {
  runtime
    .borrow()
    .view_account(account_id)
    .unwrap()
    .amount
}
