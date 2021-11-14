use crate::utils::{
  assert_eq_with_gas,
  get_account_access_key,
  get_account_balance,
  init_simulation,
  UserCreatorUtility
};
use near_sdk_sim::{call, to_yocto};
use std::rc::Rc;

#[test]
fn create_user() {
  let alice_initial_balance = to_yocto("200");
  let transfer_amount = to_yocto("100");

  let (root, runtime) = init_simulation();
  let user_creator = UserCreatorUtility::init(Rc::new(root), alice_initial_balance);
  let contract = user_creator.contract;
  let alice = user_creator.user;
  let pk = user_creator.public_key;

  let result = call!(
    alice,
    contract.create_user_account(alice.account_id.to_string(), pk.as_pk1()),
    deposit = transfer_amount
  );
  result.assert_success();

  // Check Alice balance
  let alice_balance = get_account_balance(alice.account_id.as_str(), &runtime);
  assert_eq_with_gas(
    to_yocto("100"), // 200 - 100 NEAR
    alice_balance
  );

  // Check Alice Linkdrop balance
  let new_account_id = "alice.linkdrop";
  let alice_linkdrop_balance = get_account_balance(new_account_id, &runtime);
  assert_eq_with_gas(transfer_amount, alice_linkdrop_balance);

  // Check Alice access key
  let key = get_account_access_key(new_account_id, pk.as_pk2(), &runtime);
  assert_eq!(key.is_some(), true);
}
