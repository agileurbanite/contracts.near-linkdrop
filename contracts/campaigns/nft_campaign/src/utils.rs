use crate::*;

pub fn update_drop_status(drop: &Drop, status: DropStatus) -> Drop {
  Drop {
    status,
    nft: drop.nft.clone(),
  }
}
