use crate::*;

#[near_bindgen]
impl Campaign {
  pub fn get_key_balance(&self, key: Base58PublicKey) -> U128 {
    match self.keys.get(&key.into()) {
      Some(KeyStatus::Active) => self.tokens_per_key.into(),
      _ => panic!("Public key was used or never existed"),
    }
  }
}
