use crate::*;

pub fn update_drop_status(drop: &Drop, status: DropStatus) -> Drop {
  Drop {
    status,
    nft: drop.nft.clone(),
  }
}

pub fn nft_transfer(
  collection_id: AccountId,
  token_id: TokenId,
  receiver_id: AccountId,
) -> Promise {
  Promise::new(collection_id).function_call(
    "nft_transfer".to_string(),
    json!({
      "receiver_id": receiver_id,
      "token_id": token_id,
    })
    .to_string()
    .into_bytes(),
    1,
    tgas(25),
  )
}

pub fn log_successful_nft_transfer(
  collection_id: AccountId,
  token_id: TokenId,
  receiver_id: AccountId,
) {
  env::log_str(
    format!(
      "Successfully transfer token '{}' of @{} to @{}",
      token_id, collection_id, receiver_id,
    )
    .as_str(),
  );
}

pub fn log_failed_nft_transfer(
  collection_id: AccountId,
  token_id: TokenId,
  receiver_id: AccountId,
) {
  env::panic_str(
    format!(
      "Failed to transfer token '{}' of @{} to @{}",
      token_id, collection_id, receiver_id,
    )
    .as_str(),
  );
}
