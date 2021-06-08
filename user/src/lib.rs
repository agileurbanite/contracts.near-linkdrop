// mod new;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};

// TODO Try to download contract code from linkdrop contract instead of embed it into the user contract
const NEAR_CAMPAIGN_WASM: &[u8] = include_bytes!("../../wasm/near_campaign.wasm");

setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct User {}

// TODO Check if deleting account will delete sub-accounts too
// TODO Add last_campaign_id (1,2,3...) - need to use with path for generating keys
#[near_bindgen]
impl User {
  // TODO Add private mode
  #[payable]
  pub fn create_near_campaign(
    &mut self,
    name: AccountId,
    public_key: Base58PublicKey,
    tokens_per_key: U128,
  ) -> Promise {
    let account_id = format!("{}.{}", name, env::current_account_id());

    Promise::new(account_id)
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(NEAR_CAMPAIGN_WASM.to_vec())
      .function_call(
        b"new".to_vec(),
        json!({ "tokens_per_key": tokens_per_key })
          .to_string()
          .as_bytes()
          .to_vec(),
        0,
        50_000_000_000_000,
      )
  }
}
