use super::*;
use crate::*;
use near_sdk::serde::export::TryFrom;

// TODO check access key
#[test]
fn add_one_key() {
  testing_env!(get_context());

  let mut contract = Campaign::new(U128::from(1_000_000_000_000_000_000_000_000));
  let pk = Base58PublicKey::try_from("9F9hsDH853gyZD2p1R3NXCwURNKGP616wmWwxtP2f1K5").unwrap();

  contract.add_keys(vec![pk.clone().into()]);

  assert_eq!(Some(KeyStatus::Active), contract.keys.get(&pk.clone().into()));
}
