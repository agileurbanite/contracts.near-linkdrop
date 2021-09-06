use crate::*;

#[near_bindgen]
impl User {
  // TODO: Don't need this, as there is no state with the contract. Why would this be necessary?
  #[init]
  pub fn new() -> Self {
    Self {}
  }
}
