use crate::*;

#[near_bindgen]
impl NftCampaign {
  pub fn get_drops(&self, keys: Vec<PublicKey>) -> Vec<Option<Drop>> {
    keys.into_iter().map(|key| self.drops.get(&key)).collect()
  }
}
