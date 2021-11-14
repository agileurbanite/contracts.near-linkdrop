use crate::utils::init_simulation::Runtime;
use near_sdk_sim::near_crypto::InMemorySigner;
use near_sdk_sim::{ContractAccount, UserAccount};

pub fn get_contract_account<T>(
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
