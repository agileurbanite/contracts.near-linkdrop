use crate::*;
use near_sdk::PromiseOrValue;

const ACCESS_KEY_ALLOWANCE: u128 = 90_000_000_000_000_000_000_000; // 0.09 NEAR

#[near_bindgen]
impl NftCampaign {
  pub fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool> {
    let key = msg.parse().expect("Invalid key");
    let collection_id = env::predecessor_account_id();

    assert!(
      self.collections_whitelist.contains(&collection_id),
      "Account @{} has no permission to call this method",
      collection_id
    );

    self.metadata.drops_stats.active += 1;
    self.drops.insert(
      &key,
      &Drop {
        status: DropStatus::Active,
        nft: NFT {
          token_id: token_id.clone(),
          collection_id: collection_id.clone(),
          previous_owner_id,
        },
      },
    );

    env::log_str(
      format!(
        "Get token `{}` of @{} collection from @{} ",
        token_id, collection_id, sender_id,
      )
      .as_str(),
    );

    Promise::new(env::current_account_id()).add_access_key(
      key,
      ACCESS_KEY_ALLOWANCE,
      collection_id,
      "claim".to_string(),
    );

    PromiseOrValue::Value(false)
  }
}
