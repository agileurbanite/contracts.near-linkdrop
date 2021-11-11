use crate::*;

// We expect that 'create_nft_campaign' will be called only by User contract.
// Otherwise, it will lead to an error.

#[near_bindgen]
impl NftCampaignCreator {
  #[payable]
  pub fn create_nft_campaign(&mut self, name: String, public_key: PublicKey) -> Promise {
    let campaign_id = AccountId::new_unchecked(format!("{}.{}", name, env::signer_account_id()));

    Promise::new(campaign_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key)
      .deploy_contract(NFT_CAMPAIGN.to_vec())
      .function_call(
        "new".to_string(),
        b"{}".to_vec(),
        0,
        Gas::from(25_000_000_000_000),
      )
      .then(
        Promise::new(env::current_account_id()).function_call(
          "on_create_nft_campaign".to_string(),
          json!({
            "deposit": env::attached_deposit(),
            "campaign_id": campaign_id,
          })
          .to_string()
          .into_bytes(),
          0,
          Gas(25_000_000_000_000),
        ),
      )
  }

  #[private]
  pub fn on_create_nft_campaign(&self, deposit: Balance, campaign_id: AccountId) -> Promise {
    let is_success = is_promise_success();

    if !is_success {
      Promise::new(env::signer_account_id()).transfer(deposit);
      env::panic_str(
        format!(
          "Campaign {} wasn't created. Deposit transfer back to @{}",
          campaign_id.as_str(),
          env::signer_account_id(),
        )
        .as_str(),
      );
    }

    env::log_str(format!("Campaign {} was successfully created", campaign_id.as_str()).as_str());

    Promise::new(env::signer_account_id()).function_call(
      "on_campaign_created".to_string(),
      json!({
        "campaign_id": campaign_id,
      })
      .to_string()
      .into_bytes(),
      0,
      Gas::from(25_000_000_000_000),
    )
  }
}
