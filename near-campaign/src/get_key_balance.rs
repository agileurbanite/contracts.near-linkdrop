use crate::*;

/*
  This method has the same signature as the original contract -
  it allows the contract to be compatible with NEAR Wallet
  https://github.com/near/near-linkdrop/blob/master/src/lib.rs#L180
*/

#[near_bindgen]
impl Campaign {
  pub fn get_key_balance(&self, key: Base58PublicKey) -> U128 {
    match self.keys.get(&key.0) {
      Some(KeyStatus::Active) => self.tokens_per_key.into(),
      _ => env::panic(b"Public key was used or never existed"),
    }
  }
}
