use crate::utils::deploy_contracts::deploy_nft_campaign;
use crate::utils::nft_factory::NftFactory;
use crate::utils::person::Person;
use crate::utils::{init_simulation, KeySet};
use near_crypto::InMemorySigner;
use near_sdk_sim::{call, DEFAULT_GAS};
use std::rc::Rc;

#[test]
fn test() {
  let (root, _runtime) = init_simulation();
  let root = Rc::new(root);

  let key_set = KeySet::create(0, 0);
  let (_, pk, sk) = key_set.some_keys(0);

  let alice = Person::create_alice(root.clone());
  let nft_factory = NftFactory::default_init(root.clone(), "alice");

  nft_factory.default_nft_mint(&alice.account);

  let r = nft_factory.get_nft_tokens_for_owner("alice");
  dbg!(r);

  // let result = call!(
  //   root,
  //   nft_campaign.nft_on_transfer(
  //     "bob".parse().unwrap(),
  //     "bob".parse().unwrap(),
  //     "1".to_string(),
  //     pk.to_string()
  //   ),
  //   gas = DEFAULT_GAS
  // );
  // dbg!(result);

  // Call claim
  // let signer = InMemorySigner::from_secret_key("nft_campaign".to_string(), sk);
  // nft_campaign.user_account.signer = signer.clone();
  //
  // // TODO Create another plain user account and sign tx by this account
  // let result2 = call!(
  //   nft_campaign.user_account,
  //   nft_campaign.claim("new_owner".parse().unwrap()),
  //   gas = DEFAULT_GAS
  // );
  //
  // dbg!(result2);

  // Check key
  // {
  //   // let res = runtime.borrow().view_account(&nft.account_id().as_str());
  //   let key = runtime.borrow().view_access_key("nft_campaign", &pk);
  //   dbg!(key);
  // }
}
