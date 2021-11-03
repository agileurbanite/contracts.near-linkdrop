use near_sdk::serde_json::{json, Value};
use near_sdk_sim::{to_yocto, UserAccount};
use std::rc::Rc;

lazy_static_include::lazy_static_include_bytes! {
   NFT_FACTORY_WASM => "tests/sim/utils/external_wasm/non_fungible_token.wasm",
}

pub struct NftFactory {
  pub account: UserAccount,
}

impl NftFactory {
  pub fn deploy(root_account: Rc<UserAccount>, account_id: &str, init_balance: &str) -> Self {
    let account = root_account.deploy(
      &NFT_FACTORY_WASM,
      account_id.parse().unwrap(),
      to_yocto(init_balance),
    );

    NftFactory { account }
  }

  pub fn initialize_contract(&self, owner_id: &str) {
    self.account.call(
      self.account.account_id.clone(),
      "new_default_meta",
      json!({ "owner_id": owner_id }).to_string().as_bytes(),
      DEFAULT_GAS,
      0,
    );
  }

  pub fn default_init(root_account: Rc<UserAccount>, owner_id: &str) -> Self {
    let mut nft_factory = Self::deploy(root_account, "nft_factory", "100");
    nft_factory.initialize_contract(owner_id);
    nft_factory
  }

  pub fn nft_mint(
    &self,
    signer: &UserAccount,
    token_id: &str,
    receiver_id: &str,
    title: &str,
  ) -> ExecutionResult {
    signer.call(
      self.account.account_id.clone(),
      "nft_mint",
      json!({
        "token_id": token_id,
        "receiver_id": receiver_id,
        "token_metadata": {
          "title": title,
        }
      })
      .to_string()
      .as_bytes(),
      DEFAULT_GAS,
      to_yocto("0.01"),
    )
  }

  pub fn default_nft_mint(&self, signer: &UserAccount) -> ExecutionResult {
    self.nft_mint(signer, "1", signer.account_id.as_str(), "My first NFT")
  }

  pub fn get_nft_metadata(&self) -> Value {
    self
      .account
      .view(self.account.account_id.clone(), "nft_metadata", &[])
      .unwrap_json_value()
  }

  pub fn get_nft_token(&self, token_id: &str) -> Value {
    self
      .account
      .view(
        self.account.account_id.clone(),
        "nft_token",
        json!({ "token_id": token_id }).to_string().as_bytes(),
      )
      .unwrap_json_value()
  }

  pub fn get_nft_tokens_for_owner(&self, owner_id: &str) -> Value {
    self
      .account
      .view(
        self.account.account_id.clone(),
        "nft_tokens_for_owner",
        json!({ "account_id": owner_id }).to_string().as_bytes(),
      )
      .unwrap_json_value()
  }
}
