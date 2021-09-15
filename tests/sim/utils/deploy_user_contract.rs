use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use user::UserContract;

lazy_static_include::lazy_static_include_bytes! {
   USER => "wasm/user.wasm",
}

pub fn deploy_user_contract(
  signer_account: &UserAccount,
  account_id: &str,
) -> ContractAccount<UserContract> {
  deploy!(
    contract: UserContract,
    contract_id: account_id,
    bytes: &USER,
    signer_account: signer_account,
    deposit: to_yocto("200"),
    init_method: new(),
  )
}
