use crate::*;

#[near_bindgen]
impl Campaign {
  #[init]
  pub fn new(tokens_per_key: U128) -> Self {
    Self {
      tokens_per_key: tokens_per_key.into(),
      keys_stats: KeysStats {
        total: 0,
        active: 0,
        created: 0,
        claimed: 0,
        deleted: 0,
      },
      keys: UnorderedMap::new(b"k".to_vec()),
    }
  }
}
