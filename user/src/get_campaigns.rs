use crate::*;

#[near_bindgen]
impl User {
  pub fn get_campaigns(&self) -> Vec<String> {
    self.campaigns.to_vec()
  }
}
