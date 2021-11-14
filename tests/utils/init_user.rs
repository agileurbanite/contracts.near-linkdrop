use crate::utils::{KeySet, PK};
use near_sdk_sim::{deploy, lazy_static_include, ContractAccount, UserAccount};
use std::rc::Rc;
use user::UserContract;

lazy_static_include::lazy_static_include_bytes! {
   USER_WASM_BYTES => "wasm/user.wasm"
}

const CONTRACT_ID: &str = "alice_linkdrop";

pub struct UserUtility {
  pub contract: ContractAccount<UserContract>,
  pub public_key: PK,
}

impl UserUtility {
  pub fn deploy(
    root_account: Rc<UserAccount>,
    account_id: &str,
    initial_balance: u128
  ) -> ContractAccount<UserContract> {
    let user_contract = deploy! {
      contract: UserContract,
      contract_id: account_id,
      bytes: &USER_WASM_BYTES,
      signer_account: root_account,
      deposit: initial_balance,
      init_method: new(),
    };
    user_contract
  }

  pub fn create_public_key() -> PK {
    let key_set = KeySet::create(0, 0);
    let (pk, _) = key_set.some_keys(0);
    pk
  }

  pub fn init(root_account: Rc<UserAccount>, init_balance: u128) -> Self {
    UserUtility {
      contract: Self::deploy(root_account.clone(), CONTRACT_ID, init_balance),
      public_key: Self::create_public_key(),
    }
  }
}
