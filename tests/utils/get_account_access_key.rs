use crate::utils::Runtime;
use near_crypto::PublicKey;
use near_sdk_sim::account::AccessKey;

pub fn get_account_access_key(
  account_id: &str,
  pk: PublicKey,
  runtime: &Runtime
) -> Option<AccessKey> {
  runtime
    .borrow()
    .view_access_key(account_id, &pk)
}
