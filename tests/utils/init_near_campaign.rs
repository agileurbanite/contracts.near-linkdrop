use crate::utils::{CommonUtils, KeySet, Person, Runtime};
use near_campaign::CampaignContract as NearCampaign;
use near_crypto::{InMemorySigner, SecretKey};
use near_sdk::{AccountId, PublicKey};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{deploy, lazy_static_include, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS};
use std::rc::Rc;
use user::UserContract;

lazy_static_include::lazy_static_include_bytes! {
   USER_WASM_BYTES => "wasm/user.wasm",
   NEAR_CAMPAIGN_WASM_BYTES => "wasm/near_campaign.wasm",
   EXTERNAL_LINKDROP_WASM_BYTES => "tests/utils/external_wasm/external_linkdrop.wasm"
}

const USER_CONTRACT_ID: &str = "alice_linkdrop";
const CONTRACT_ID: &str = "near_campaign";

pub struct NearCampaignUtility {
  pub root_account: Rc<UserAccount>,
  pub runtime: Runtime,
  pub contract: ContractAccount<NearCampaign>,
  pub keys: KeySet,
  pub user_contract: Option<ContractAccount<UserContract>>,
}

impl NearCampaignUtility {
  pub fn deploy_near_campaign(
    root_account: Rc<UserAccount>,
    account_id: &str,
    initial_balance: u128,
    total_keys: u64,
    tokens_per_key: &str
  ) -> ContractAccount<NearCampaign> {
    let near_campaign = deploy! {
      contract: NearCampaign,
      contract_id: account_id,
      bytes: &NEAR_CAMPAIGN_WASM_BYTES,
      signer_account: root_account,
      deposit: initial_balance,
      init_method: new(
        1,
        total_keys,
        U128::from(to_yocto(tokens_per_key)),
        "testnet".parse().unwrap(),
        "alice.linkdrop".parse().unwrap()
      )
    };
    near_campaign
  }

  pub fn deploy_user(
    root_account: Rc<UserAccount>,
    account_id: &str,
    initial_balance: u128
  ) -> ContractAccount<UserContract> {
    let user_contract = deploy! {
      contract: UserContract,
      contract_id: account_id,
      bytes: &USER_WASM_BYTES,
      signer_account: root_account,
      deposit: initial_balance,
      init_method: new(),
    };
    user_contract
  }

  pub fn create_keys(from: usize, to: usize) -> KeySet {
    KeySet::create(from, to)
  }

  pub fn get_contract_storage_usage(&self) -> u64 {
    let runtime = self.root_account.borrow_runtime();
    runtime
      .view_account(self.contract.account_id().as_str())
      .unwrap()
      .storage_usage
  }

  pub fn add_keys_from_array(&mut self, keys: Vec<PublicKey>) {
    self.contract.user_account.call(
      self.contract.account_id().clone(),
      "add_keys",
      json!({ "keys": keys }).to_string().as_bytes(),
      100_000_000_000_000,
      0,
    );
  }

  pub fn add_keys(&mut self) {
    self.add_keys_from_array(self.keys.public_keys());
  }

  pub fn set_signer_to_claim(&mut self, index: usize) {
    let (_, sk) = self.keys.some_keys(index);
    let claim_signer = InMemorySigner::from_secret_key(
      self.contract.account_id().into(),
      sk
    );
    self.contract.user_account.signer = claim_signer;
  }

  pub fn create_user(&self, init_balance: &str) -> UserAccount {
    Person::create_alice(self.root_account.clone(), init_balance).account
  }

  pub fn pre_create_account_and_claim(
    &mut self,
    new_account_id: AccountId,
    pk_index1: usize,
    pk_index2: usize
  ) {
    self.set_signer_to_claim(pk_index1);
    let (pk_new, _) = self.keys.some_keys(pk_index2);

    self.contract.user_account.call(
      self.contract.account_id().clone(),
      "create_account_and_claim",
      &json!({
        "new_account_id": new_account_id.to_string(),
        "new_public_key": pk_new.as_pk1()
      })
        .to_string()
        .into_bytes(),
      100000000000000, // 100 TGas
      0
    );
  }

  pub fn pre_claim_one_link(&mut self, account_id: AccountId, pk_index: usize) {
    let old_signer = self.contract.user_account.signer.clone();
    self.set_signer_to_claim(pk_index);

    self.contract.user_account.call(
      self.contract.account_id().clone(),
      "claim",
      &json!({
        "account_id": account_id.to_string(),
      }).to_string().into_bytes(),
      100000000000000, // 100 TGas
      0
    );

    self.contract.user_account.signer = old_signer;
  }

  pub fn init_external_linkdrop(&self) {
    self.root_account.deploy(
      &EXTERNAL_LINKDROP_WASM_BYTES,
      "testnet".parse().unwrap(),
      to_yocto("5"),
    );
  }

  pub fn create_near_campaign(
    user_contract: &ContractAccount<UserContract>,
    initial_campaign_balance: u128,
    total_keys: u64,
    tokens_per_key: &str,
    public_key: PublicKey,
  ) {
    user_contract.user_account.call(
      user_contract.account_id().clone(),
      "create_near_campaign",
      &json!({
        "name": CONTRACT_ID.to_string(),
        "public_key": public_key,
        "total_keys": total_keys,
        "tokens_per_key": U128::from(to_yocto(tokens_per_key)),
        "account_creator": "testnet".to_string()
      }).to_string().into_bytes(),
      DEFAULT_GAS,
      initial_campaign_balance
    );
  }

  pub fn create_near_campaign_contract_account(
    secret_key: SecretKey,
    runtime: Runtime
  ) -> ContractAccount<NearCampaign> {
    let campaign_account_id = format!("{}.{}", CONTRACT_ID, USER_CONTRACT_ID);
    CommonUtils::retrieve_contract_account(
      campaign_account_id.as_str(),
      secret_key.to_string().as_str(),
      runtime,
      NearCampaign {
        account_id: campaign_account_id.parse().unwrap(),
      }
    )
  }

  pub fn init_near_campaign(
    initial_balance: u128,
    total_keys: u64,
    tokens_per_key: &str,
    keys_from: usize,
    keys_to: usize
  ) -> Self {
    let (root, runtime) = CommonUtils::init_simulation();

    NearCampaignUtility {
      root_account: root.clone(),
      runtime,
      contract: Self::deploy_near_campaign(
        root.clone(),
        CONTRACT_ID,
        initial_balance,
        total_keys,
        tokens_per_key
      ),
      keys: Self::create_keys(keys_from, keys_to),
      user_contract: None,
    }
  }

  pub fn init_user_and_near_campaign(
    initial_user_balance: u128,
    initial_campaign_balance: u128,
    total_keys: u64,
    tokens_per_key: &str,
    keys_from: usize,
    keys_to: usize
  ) -> Self {
    let (root, runtime) = CommonUtils::init_simulation();

    let user_contract = Self::deploy_user(
      root.clone(),
      USER_CONTRACT_ID,
      initial_user_balance
    );

    let (public_key, secret_key) = KeySet::create(0, 0).some_keys(0);
    Self::create_near_campaign(
      &user_contract,
      initial_campaign_balance,
      total_keys,
      tokens_per_key,
      public_key.as_pk1(),
    );

    NearCampaignUtility {
      root_account: root.clone(),
      runtime: runtime.clone(),
      contract: Self::create_near_campaign_contract_account(secret_key, runtime),
      keys: Self::create_keys(keys_from, keys_to),
      user_contract: Some(user_contract),
    }
  }
}
