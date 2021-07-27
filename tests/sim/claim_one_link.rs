use crate::utils::{get_public_keys, init};
use near_crypto::{InMemorySigner, PublicKey, SecretKey, Signer};
use near_sdk_sim::{call, to_yocto};
use std::str::FromStr;

// TODO use keys via 'get_public_keys'
const PK: &str = "8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe";
const SK: &str =
  "39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y";

#[test]
fn claim_one_link() {
  let (root, mut near_campaign) = init("5");
  let bob = root.create_user("bob".to_string(), to_yocto("10"));
  let public_keys = get_public_keys(0, 0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  // Check if the key was added as functional call assess key
  {
    let runtime = root.borrow_runtime();
    let pk = PublicKey::from_str(PK).unwrap();
    let key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk);
    assert_eq!(key.is_some(), true);
  }

  // We want to sing transaction by new key;
  let claim_signer =
    InMemorySigner::from_secret_key(bob.account_id(), SecretKey::from_str(SK).unwrap());

  near_campaign.user_account.signer = claim_signer.clone();

  call!(
    near_campaign.user_account,
    near_campaign.claim(bob.account_id())
  );
  assert_eq!(to_yocto("15"), bob.account().unwrap().amount);

  // Used key should not exist after the successful 'claim'
  {
    let runtime = root.borrow_runtime();
    let key = runtime.view_access_key(
      near_campaign.account_id().as_str(),
      &claim_signer.public_key(),
    );
    assert_eq!(key.is_none(), true);
  }
}
