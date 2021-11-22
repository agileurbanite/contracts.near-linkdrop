use crate::utils::{CommonUtils, KeySet, Person, Runtime};
use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use nft_campaign::NftCampaignContract;
use std::rc::Rc;
use near_crypto::InMemorySigner;

lazy_static_include::lazy_static_include_bytes! {
  NFT_CAMPAIGN_WASM_BYTES => "wasm/nft_campaign.wasm",
}

pub struct NftCampaignUtility {
  pub root_account: Rc<UserAccount>,
  pub runtime: Runtime,
  pub contract: ContractAccount<NftCampaignContract>,
  pub keys: KeySet,
}

impl NftCampaignUtility {
  pub fn deploy_nft_campaign(
    signer_account: Rc<UserAccount>,
    account_id: &str,
  ) -> ContractAccount<NftCampaignContract> {
    deploy!(
      contract: NftCampaignContract,
      contract_id: account_id,
      bytes: &NFT_CAMPAIGN_WASM_BYTES,
      signer_account: signer_account,
      deposit: to_yocto("200"),
      init_method: new(),
    )
  }

  pub fn create_keys(from: usize, to: usize) -> KeySet {
    KeySet::create(from, to)
  }

  pub fn create_user(&self, name: &str) -> UserAccount {
    Person::new(self.root_account.clone(), name, "100").account
  }

  pub fn set_signer_to_claim(&mut self, index: usize) {
    let (_, sk) = self.keys.some_keys(index);
    let claim_signer = InMemorySigner::from_secret_key(
      self.contract.account_id().into(),
      sk
    );
    self.contract.user_account.signer = claim_signer;
  }

  pub fn init_nft_campaign(
    account_id: &str,
    keys_from: usize,
    keys_to: usize
  ) -> Self {
    let (root, runtime) = CommonUtils::init_simulation();

    NftCampaignUtility {
      root_account: root.clone(),
      runtime,
      contract: Self::deploy_nft_campaign(root, account_id),
      keys: Self::create_keys(keys_from, keys_to),
    }
  }
}
