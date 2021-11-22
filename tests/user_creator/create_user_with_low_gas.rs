use crate::utils::{CommonUtils, UserCreatorUtility};
use near_sdk::serde_json::json;
use near_sdk_sim::to_yocto;

#[test]
fn create_user_with_low_gas() {
  let alice_initial_balance = "200";
  let transfer_amount = to_yocto("100");

  let (root, runtime) = CommonUtils::init_simulation();
  let user_creator = UserCreatorUtility::init(root, alice_initial_balance);
  let contract = user_creator.contract;
  let alice = user_creator.user;
  let pk = user_creator.public_key;

  let contract_balance_start = contract.account().unwrap().amount;

  let result = alice.call(
    contract.account_id(),
    "create_user_account",
    &json!({
      "name": alice.account_id.to_string(),
      "public_key": pk.as_pk1()
    })
      .to_string()
      .into_bytes(),
    1_000_000_000, // 1 GigaGas
    transfer_amount, // deposit
  );
  // No successful outcome is expected
  assert!(!result.is_ok());

  // One error must occur while running the method
  CommonUtils::assert_one_promise_error(result.clone(), "Exceeded the prepaid gas");

  // Alice's balance has not changed
  let alice_balance = CommonUtils::retrieve_account_balance(alice.account_id.as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(to_yocto(alice_initial_balance), alice_balance);

  // The balance of the contract has not changed
  let contract_balance_end = CommonUtils::retrieve_account_balance(contract.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(contract_balance_start, contract_balance_end);
}
