use near_sdk_sim::{lazy_static_include, to_yocto, UserAccount};
use std::rc::Rc;

lazy_static_include::lazy_static_include_bytes! {
   NFT_HOLDER_WASM => "tests/sim/utils/external_wasm/non_fungible_token.wasm",
}

type RootAccount = Rc<UserAccount>;

pub struct NFTHolder {
  pub root_account: RootAccount,
  pub account: UserAccount,
}

impl NFTHolder {
  pub fn deploy(root_account: RootAccount, account_id: &str, init_balance: &str) -> Self {
    let account = root_account.deploy(
      &NFT_HOLDER_WASM,
      account_id.parse().unwrap(),
      to_yocto(init_balance),
    );

    NFTHolder {
      root_account,
      account,
    }
  }

  pub fn initialize_contract(&mut self) {

  }

  pub fn default_init(root_account: RootAccount) -> Self {
    Self::deploy(root_account, "nft_holder", "100")
  }
}
