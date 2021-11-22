use near_crypto::PublicKey;
use near_sdk_sim::account::AccessKey;
use near_sdk_sim::near_crypto::InMemorySigner;
use near_sdk_sim::runtime::{GenesisConfig, init_runtime, RuntimeStandalone};
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::types::Balance;
use near_sdk_sim::{to_yocto, ContractAccount, UserAccount, ExecutionResult};
use std::cell::RefCell;
use std::rc::Rc;

pub type Runtime = Rc<RefCell<RuntimeStandalone>>;

pub struct CommonUtils { }

impl CommonUtils {
  pub fn init_simulation() -> (Rc<UserAccount>, Runtime) {
    let mut genesis = GenesisConfig::default();
    genesis.gas_limit = 900 * 10u64.pow(12);
    let (runtime, signer, root_account_id) = init_runtime(Some(genesis));
    let wrapped_runtime = Rc::new(RefCell::new(runtime));
    (
      Rc::new(UserAccount::new(&wrapped_runtime, root_account_id, signer)),
      wrapped_runtime,
    )
  }

  pub fn assert_almost_eq_with_max_delta(left: u128, right: u128, max_delta: u128) {
    assert!(
      std::cmp::max(left, right) - std::cmp::min(left, right) <= max_delta,
      "{}",
      format!(
        "Left {} is not even close to Right {} within delta {}",
        left, right, max_delta
      )
    );
  }

  pub fn assert_eq_with_gas(left: u128, right: u128) {
    Self::assert_almost_eq_with_max_delta(left, right, to_yocto("0.03")); // 300 Tgas
  }

  pub fn assert_one_promise_error(promise_result: ExecutionResult, expected_error_message: &str) {
    assert_eq!(promise_result.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) =
    &promise_result.promise_errors().remove(0).unwrap().outcome().status
    {
      assert!(execution_error.to_string().contains(expected_error_message));
    } else {
      unreachable!();
    }
  }

  pub fn retrieve_account_access_key(
    account_id: &str,
    pk: PublicKey,
    runtime: &Runtime
  ) -> Option<AccessKey> {
    runtime
      .borrow()
      .view_access_key(account_id, &pk)
  }

  pub fn retrieve_account_balance(account_id: &str, runtime: &Runtime) -> Balance {
    runtime
      .borrow()
      .view_account(account_id)
      .unwrap()
      .amount
  }

  pub fn retrieve_contract_account<T>(
    account_id: &str,
    secret_key: &str,
    runtime: Runtime,
    contract: T,
  ) -> ContractAccount<T> {
    let signer = InMemorySigner::from_secret_key(account_id.to_string(), secret_key.parse().unwrap());

    ContractAccount {
      user_account: UserAccount::new(&runtime, account_id.parse().unwrap(), signer),
      contract,
    }
  }
}
