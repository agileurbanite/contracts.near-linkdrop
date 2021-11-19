use crate::utils::{CommonUtils, UserCreatorUtility};
use near_sdk_sim::{call, to_yocto};

#[test]
#[should_panic(expected = r#"NotEnoughBalance"#)]
fn create_user_not_enough_balance() {
  let alice_initial_balance = "200";
  let transfer_amount = to_yocto("200");

  let (root, _runtime) = CommonUtils::init_simulation();
  let user_creator = UserCreatorUtility::init(root, alice_initial_balance);
  let contract = user_creator.contract;
  let alice = user_creator.user;
  let pk = user_creator.public_key;

  call!(
    alice,
    contract.create_user_account(alice.account_id.to_string(), pk.as_pk1()),
    deposit = transfer_amount
  );
}
