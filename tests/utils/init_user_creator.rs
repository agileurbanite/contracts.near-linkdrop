use crate::utils::{KeySet, Person, PK};
use near_crypto::SecretKey;
use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use std::rc::Rc;
use user_creator::UserCreatorContract;

lazy_static_include::lazy_static_include_bytes! {
   USER_CREATOR_WASM_BYTES => "wasm/user_creator.wasm"
}

const CONTRACT_ID: &str = "linkdrop";

pub struct UserCreatorUtility {
  pub contract: ContractAccount<UserCreatorContract>,
  pub user: UserAccount,
  pub public_key: PK,
  pub secret_key: SecretKey
}

impl UserCreatorUtility {
  pub fn deploy(
    root_account: Rc<UserAccount>,
    account_id: &str
  ) -> ContractAccount<UserCreatorContract> {
    let user_creator = deploy! {
      contract: UserCreatorContract,
      contract_id: account_id,
      bytes: &USER_CREATOR_WASM_BYTES,
      signer_account: root_account,
    };
    user_creator
  }

  pub fn create_user(root_account: Rc<UserAccount>, init_balance: &str) -> UserAccount {
    Person::create_alice(root_account, init_balance).account
  }

  pub fn create_drop_user(&self) -> UserAccount {
    let user_account_id = format!("{}.{}", self.user.account_id(), self.contract.account_id());
    self.contract.user_account.create_user(user_account_id.parse().unwrap(), to_yocto("10"))
  }

  pub fn create_keys() -> (PK, SecretKey) {
    let key_set = KeySet::create(0, 0);
    key_set.some_keys(0)
  }

  pub fn init(root_account: Rc<UserAccount>, init_balance: &str) -> Self {
    let (pk, sk) = Self::create_keys();
    UserCreatorUtility {
      contract: Self::deploy(root_account.clone(), CONTRACT_ID),
      user: Self::create_user(root_account, init_balance),
      public_key: pk,
      secret_key: sk,
    }
  }
}
