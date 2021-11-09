use near_sdk_sim::runtime::{init_runtime, RuntimeStandalone};
use near_sdk_sim::UserAccount;
use std::cell::RefCell;
use std::rc::Rc;

pub type Runtime = Rc<RefCell<RuntimeStandalone>>;

pub fn init_simulation() -> (UserAccount, Runtime) {
  let (runtime, signer, root_account_id) = init_runtime(None);
  let wrapped_runtime = Rc::new(RefCell::new(runtime));
  (
    UserAccount::new(&wrapped_runtime, root_account_id, signer),
    wrapped_runtime,
  )
}
