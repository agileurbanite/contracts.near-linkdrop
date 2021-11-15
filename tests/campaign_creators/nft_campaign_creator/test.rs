use crate::utils::deploy_contracts::deploy_nft_campaign;
use crate::utils::nft_factory::NftFactory;
use crate::utils::person::Person;
use crate::utils::{CommonUtils, KeySet};
use near_crypto::InMemorySigner;
use near_sdk_sim::account::{AccessKey, AccessKeyPermission, FunctionCallPermission};
use near_sdk_sim::{call,view, to_yocto, DEFAULT_GAS};

#[test]
fn test() {
  let (root, _runtime) = CommonUtils::init_simulation();

  let key_set = KeySet::create(0, 2);
  let (pk, sk) = key_set.some_keys(0);
  let (pk2, sk2) = key_set.some_keys(1);

  let alice = Person::create_alice(root.clone());
  let nft_factory = NftFactory::default_init(root.clone(), "alice");
  let mut nft_campaign = deploy_nft_campaign(&root, "nft_campaign");
}
