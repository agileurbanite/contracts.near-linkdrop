use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub version: String,
}

#[near_bindgen]
impl User {
  pub fn get_user_metadata(self) -> Metadata {
    Metadata {
      version: self.version,
    }
  }
}
