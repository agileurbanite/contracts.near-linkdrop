use crate::*;

#[near_bindgen]
impl NftCampaign {
  #[private]
  pub fn claim(&mut self, _receiver_id: AccountId) {
    let key = env::signer_account_pk();
    // env::log_str(format!("key {}", String::from(&key)).as_str());
    // env::log_str(receiver_id.as_str());

    let drop = self.drops.get(&key).expect("No drop associated with key");
    env::log_str(drop.nft.contract_id.as_str());
    // Promise::new(receiver_id)
    //   .transfer(1)
  }
}
