use crate::utils::{
  assert_eq_with_gas,
  assert_one_promise_error,
  get_account_balance,
  init_simulation,
  UserCreatorUtility
};
use near_sdk_sim::{call, to_yocto};
use std::rc::Rc;

#[test]
fn create_user_name_is_already_exists() {
  let alice_initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("100");

  let (root, runtime) = init_simulation();
  let user_creator = UserCreatorUtility::init(Rc::new(root), alice_initial_balance);

  // Pre-create Alice Linkdrop account
  user_creator.create_drop_user();

  let contract = user_creator.contract;
  let alice = user_creator.user;
  let pk = user_creator.public_key;
  let contract_balance_start = contract.account().unwrap().amount;

  let result = call!(
    alice,
    contract.create_user_account(alice.account_id.to_string(), pk.as_pk1()),
    deposit = transfer_amount
  );
  result.assert_success();

  // One error must occur while running the method
  assert_one_promise_error(
    result.clone(),
    "Can't create a new account \"alice.linkdrop\", because it already exists"
  );

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("Is user created: false"));

  // Alice's balance has not changed
  let alice_balance = get_account_balance(alice.account_id.as_str(), &runtime);
  assert_eq_with_gas(alice_initial_balance, alice_balance);

  // The balance of the contract has not changed
  let contract_balance_end = get_account_balance(contract.account_id().as_str(), &runtime);
  assert_eq_with_gas(contract_balance_start, contract_balance_end);
}
