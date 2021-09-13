use user::UserContract;
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{
  deploy, init_simulator, lazy_static_include, ContractAccount, UserAccount
};

lazy_static_include::lazy_static_include_bytes! {
   USER_WASM_BYTES => "wasm/user.wasm"
}

const CONTRACT_ID: &str = "alice_linkdrop";

pub fn init_user_contract(initial_balance: u128) -> (UserAccount, ContractAccount<UserContract>) {
  let genesis = GenesisConfig::default();
  let root = init_simulator(Some(genesis));

  let user_contract = deploy! {
    contract: UserContract,
    contract_id: CONTRACT_ID,
    bytes: &USER_WASM_BYTES,
    signer_account: root,
    deposit: initial_balance,
    init_method: new(),
  };

  (root, user_contract)
}
