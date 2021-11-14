use crate::utils::{deploy_nft_campaign, init_simulation, KeySet, NftFactory, Person};
use near_crypto::InMemorySigner;
use near_sdk_sim::{call, view, DEFAULT_GAS};
use std::rc::Rc;

#[test]
fn test() {
  let (root, _runtime) = init_simulation();
  let root = Rc::new(root);

  let key_set = KeySet::create(0, 2);
  let (key, sk) = key_set.some_keys(0);
  let (key2, _sk2) = key_set.some_keys(1);

  let alice = Person::create_alice(root.clone());
  let nft_factory = NftFactory::default_init(root.clone(), "alice");
  let mut nft_campaign = deploy_nft_campaign(&root, "nft_campaign");

  nft_factory.default_nft_mint(&alice.account);
  nft_factory.nft_transfer_call(&alice.account, "nft_campaign", "1", key.as_pk2().to_string().as_str());

  // dbg!(nft_factory.get_nft_token("1"));

  // Call claim
  let signer = InMemorySigner::from_secret_key("nft_campaign".to_string(), sk);
  nft_campaign.user_account.signer = signer.clone();

  let result2 = call!(
    nft_campaign.user_account,
    nft_campaign.claim("new_owner".parse().unwrap()),
    gas = DEFAULT_GAS
  );

  dbg!(&result2);
  // dbg!(&result2.promise_results());

  let view1 = view!(nft_campaign.get_drops(vec![key.as_pk1(), key2.as_pk1()]));
  dbg!(view1.unwrap_json_value());

  // Check key
  // {
  //   // let res = runtime.borrow().view_account(&nft.account_id().as_str());
  //   let key = runtime.borrow().view_access_key("nft_campaign", &pk);
  //   dbg!(key);
  // }
}
