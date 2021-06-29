use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ClearStatus {
  Completed(bool),
}

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn clear_state(&mut self, keys: Vec<Base58PublicKey>) -> ClearStatus {
    keys.iter().for_each(|pk| {
      let key = pk.clone().into();
      self.keys.remove(&key);
    });

    ClearStatus::Completed(self.keys.is_empty())
  }
}
