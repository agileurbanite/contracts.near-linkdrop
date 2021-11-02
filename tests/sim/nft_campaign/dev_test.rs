use crate::utils::deploy_contracts::{deploy_nft_campaign};
use crate::utils::{init_simulation, nft_holder::NFTHolder, KeySet};
use near_crypto::InMemorySigner;
use near_sdk_sim::{call, DEFAULT_GAS};
use std::rc::Rc;

#[test]
fn test() {
  let (root, runtime) = init_simulation();
  let root = Rc::new(root);

  let key_set = KeySet::create(0, 0);
  let (_, pk, sk) = key_set.some_keys(0);

  let mut nft_holder = NFTHolder::default_init(root.clone());
  // let mut nft_campaign = deploy_nft_campaign(&root, "nft_campaign");

  let resp = nft_holder
    .account
    .view(nft_holder.account.account_id.clone(), "nft_metadata", &[]);

  dbg!(resp);

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
