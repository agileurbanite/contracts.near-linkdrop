use crate::*;

#[near_bindgen]
impl User {
  #[init]
  pub fn new() -> Self {
    Self {
      campaigns: UnorderedSet::new(b"c"),
      version: "2.0.0".to_string(),
    }
  }
}
