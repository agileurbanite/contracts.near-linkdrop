use crate::*;

/*
  get_keys allows us to check the status of specific keys -
  if a key was claimed or it is still active etc
 */

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
          .expect("No key found in the state"), // TODO return None instead of panic
      })
      .collect()
  }
}
