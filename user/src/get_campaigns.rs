use crate::*;

#[near_bindgen]
impl User {
  pub fn get_campaigns(&self) -> Vec<AccountId> {
    self.campaigns.to_vec()
  }
}
