use crate::utils::init_simulation;
use near_crypto::{InMemorySigner, SecretKey};
use near_sdk::json_types::U128;
use near_sdk::{PublicKey, AccountId};
use near_sdk_sim::{call, deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use user::UserContract;

lazy_static_include::lazy_static_include_bytes! {
   USER_WASM_BYTES => "wasm/user.wasm"
}

const USER_CONTRACT_ID: &str = "alice_linkdrop";
const CAMPAIGN_CONTRACT_ID: &str = "near_campaign";

pub fn init_user_and_near_campaign(
  initial_user_balance: u128,
  initial_campaign_balance: u128,
  total_keys: u64,
  tokens_per_key: &str,
  public_key: PublicKey,
  secret_key: SecretKey
) -> (UserAccount, ContractAccount<UserContract>, UserAccount) {
  let (root, wrapped_runtime) = init_simulation();

  let user = deploy! {
    contract: UserContract,
    contract_id: USER_CONTRACT_ID,
    bytes: &USER_WASM_BYTES,
    signer_account: root,
    deposit: initial_user_balance,
    init_method: new(),
  };

  call!(
    user.user_account,
    user.create_near_campaign(
      CAMPAIGN_CONTRACT_ID.to_string(),
      public_key,
      total_keys,
      U128::from(to_yocto(tokens_per_key)),
      "testnet".parse().unwrap()
    ),
    deposit = initial_campaign_balance
  );

  let campaign_account_id = format!("{}.{}", CAMPAIGN_CONTRACT_ID, USER_CONTRACT_ID);
  let signer = InMemorySigner::from_secret_key(
    campaign_account_id.clone(),
    secret_key
  );
  let near_campaign = UserAccount::new(
    &wrapped_runtime,
    AccountId::new_unchecked(campaign_account_id),
    signer
  );

  (root, user, near_campaign)
}
