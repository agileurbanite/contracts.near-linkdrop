use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Key {
  pk: Base58PublicKey,
  status: KeyStatus,
}

#[near_bindgen]
impl Campaign {
  pub fn get_keys(&self, keys: Vec<Base58PublicKey>) -> Vec<Key> {
    keys
      .iter()
      .map(|pk| Key {
        pk: pk.clone(),
        status: self
          .keys
          .get(&pk.clone().into())
          .expect("Don't have such key in state"),
      })
      .collect()
  }
}
