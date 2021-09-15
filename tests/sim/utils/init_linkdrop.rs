use linkdrop::LinkdropContract;
use near_sdk::AccountId;
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{deploy, init_simulator, lazy_static_include, ContractAccount, UserAccount};

lazy_static_include::lazy_static_include_bytes! {
   LINKDROP_WASM_BYTES => "wasm/linkdrop.wasm"
}

const CONTRACT_ID: &str = "linkdrop";

pub fn init_linkdrop(
  initial_balance: u128,
) -> (UserAccount, ContractAccount<LinkdropContract>, UserAccount) {
  let genesis = GenesisConfig::default();
  let root = init_simulator(Some(genesis));

  let linkdrop = deploy! {
    contract: LinkdropContract,
    contract_id: CONTRACT_ID,
    bytes: &LINKDROP_WASM_BYTES,
    signer_account: root,
  };

  let alice = root.create_user(
    AccountId::new_unchecked("alice".to_string()),
    initial_balance,
  );

  (root, linkdrop, alice)
}
