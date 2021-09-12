use crate::*;

pub fn create_campaign() -> Campaign {
  Campaign::new(
    1,
    U128::from(1_000_000_000_000_000_000_000_000),
    "testnet".parse().unwrap()
  )
}
