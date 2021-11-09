use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount};
use nft_campaign::NftCampaignContract;

lazy_static_include::lazy_static_include_bytes! {
   NFT_CAMPAIGN => "wasm/nft_campaign.wasm",
}

pub fn deploy_nft_campaign(
  signer_account: &UserAccount,
  account_id: &str,
) -> ContractAccount<NftCampaignContract> {
  deploy!(
    contract: NftCampaignContract,
    contract_id: account_id,
    bytes: &NFT_CAMPAIGN,
    signer_account: signer_account,
    deposit: to_yocto("200"),
    init_method: new(),
  )
}
