use crate::*;
use near_sdk::PromiseOrValue;

// TODO Do we need to write logic here or it is better to just call add_drop func from this method?
#[near_bindgen]
impl NftCampaign {
  pub fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    _previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool> {
    // validate Key
    // validate that predecessor_account_id() is a NFT contract
    let key = msg.parse().expect("Invalid key");

    self.drops.insert(
      &key,
      &Drop {
        status: "active".to_string(),
        nft: NFT {
          token_id: token_id.clone(),
          contract_id: env::predecessor_account_id(),
        },
      },
    );

    env::log_str(
      format!(
        "Key {}, nft: token_id = {}, sender_id = {}",
        msg, token_id, sender_id
      )
      .as_str(),
    );

    Promise::new(env::current_account_id()).add_access_key(
      key,
      1_000_000_000_000_000_000_000_000,
      env::current_account_id(),
      "claim".to_string(),
    );

    PromiseOrValue::Value(false)
  }
}
