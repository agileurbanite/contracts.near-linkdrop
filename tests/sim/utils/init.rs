use near_campaign::CampaignContract as NearCampaign;
use near_sdk::json_types::U128;
use near_sdk_sim::near_crypto::{InMemorySigner, KeyType};
use near_sdk_sim::runtime::{GenesisConfig, RuntimeStandalone};
use near_sdk_sim::{
  deploy, lazy_static_include, runtime::init_runtime, to_yocto, ContractAccount, UserAccount,
};
use std::cell::RefCell;
use std::rc::Rc;
use user::UserContract;

lazy_static_include::lazy_static_include_bytes! {
   NEAR_CAMPAIGN => "wasm/near_campaign.wasm",
   USER => "wasm/user.wasm",
}

type Runtime = Rc<RefCell<RuntimeStandalone>>;

pub fn init_sim(genesis_config: Option<GenesisConfig>) -> (UserAccount, Runtime) {
  let (runtime, signer, root_account_id) = init_runtime(genesis_config);
  let wrapped_runtime = Rc::new(RefCell::new(runtime));

  (
    UserAccount::new(&wrapped_runtime, root_account_id, signer),
    wrapped_runtime,
  )
}

pub fn init(tokens_per_key: &str) -> (UserAccount, Runtime, ContractAccount<NearCampaign>) {
  let genesis = GenesisConfig::default();
  let (root, runtime) = init_sim(Some(genesis));

  let campaign_id = 1;
  let tokens_per_keys = U128::from(to_yocto(tokens_per_key));
  let account_creator = "testnet".parse().unwrap();

  let near_campaign = deploy!(
    contract: NearCampaign,
    contract_id: "near_campaign",
    bytes: &NEAR_CAMPAIGN,
    signer_account: root,
    init_method: new(campaign_id, tokens_per_keys, account_creator),
  );

  (root, runtime, near_campaign)
}

pub fn deploy_user_contract(signer_account: &UserAccount) -> ContractAccount<UserContract> {
  deploy!(
    contract: UserContract,
    contract_id: "bob",
    bytes: &USER,
    signer_account: signer_account,
    deposit: to_yocto("100"),
    init_method: new(),
  )
}

pub fn get_contract_account<T>(
  account_id: &str,
  runtime: Runtime,
  contract: T,
) -> ContractAccount<T> {
  let signer = InMemorySigner::from_seed(account_id, KeyType::ED25519, account_id);

  ContractAccount {
    user_account: UserAccount::new(&runtime, account_id.parse().unwrap(), signer),
    contract,
  }
}
