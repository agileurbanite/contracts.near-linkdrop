use crate::*;

/*
 get_keys allows us to check the status of specific keys -
 if a key was claimed or it is still active etc
*/

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Key {
  pk: Base58PublicKey,
  status: Option<KeyStatus>,
}

#[near_bindgen]
impl Campaign {
  pub fn get_keys(&self, keys: Vec<Base58PublicKey>) -> Vec<Key> {
    keys
      .into_iter()
      .map(|pk| Key {
        status: self.keys.get(&pk.0),
        pk,
      })
      .collect()
  }
}
