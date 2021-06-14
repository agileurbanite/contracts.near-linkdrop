use crate::*;

#[near_bindgen]
impl User {
  #[init]
  pub fn new() -> Self {
    Self {
      campaigns: Vector::new(b'c'),
    }
  }
}
