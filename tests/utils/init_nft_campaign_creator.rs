use crate::utils::{KeySet, PK};
use near_crypto::SecretKey;
use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use nft_campaign_creator::NftCampaignCreatorContract;
use std::rc::Rc;

lazy_static_include::lazy_static_include_bytes! {
   NFT_CAMPAIGN_CREATOR_WASM_BYTES => "wasm/nft_campaign_creator.wasm"
}

const CONTRACT_ID: &str = "nft_campaign_creator";

pub struct NftCampaignCreatorUtility {
  pub contract: ContractAccount<NftCampaignCreatorContract>,
  pub public_key: PK,
}

impl NftCampaignCreatorUtility {
  pub fn deploy_nft_campaign_creator(
    root_account: Rc<UserAccount>,
    account_id: &str,
    initial_balance: &str
  ) -> ContractAccount<NftCampaignCreatorContract> {
    let nft_campaign_creator = deploy! {
      contract: NftCampaignCreatorContract,
      contract_id: account_id,
      bytes: &NFT_CAMPAIGN_CREATOR_WASM_BYTES,
      signer_account: root_account,
      deposit: to_yocto(initial_balance),
    };
    nft_campaign_creator
  }

  pub fn create_keys() -> (PK, SecretKey) {
    let key_set = KeySet::create(0, 0);
    key_set.some_keys(0)
  }

  pub fn init_nft_campaign_creator(
    root_account: Rc<UserAccount>,
    init_contract_balance: &str,
  ) -> Self {
    let (pk, _) = Self::create_keys();
    NftCampaignCreatorUtility {
      contract: Self::deploy_nft_campaign_creator(
        root_account.clone(),
        CONTRACT_ID,
        init_contract_balance
      ),
      public_key: pk,
    }
  }
}
