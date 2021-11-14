use near_sdk_sim::runtime::{GenesisConfig, init_runtime, RuntimeStandalone};
use near_sdk_sim::UserAccount;
use std::cell::RefCell;
use std::rc::Rc;

pub type Runtime = Rc<RefCell<RuntimeStandalone>>;

pub fn init_simulation() -> (UserAccount, Runtime) {
  let mut genesis = GenesisConfig::default();
  genesis.gas_limit = 900 * 10u64.pow(12);
  let (runtime, signer, root_account_id) = init_runtime(Some(genesis));
  let wrapped_runtime = Rc::new(RefCell::new(runtime));
  (
    UserAccount::new(&wrapped_runtime, root_account_id, signer),
    wrapped_runtime,
  )
}
