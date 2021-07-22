use near_campaign::Campaign;
use near_sdk_sim::{
  deploy, init_simulator, lazy_static_include, to_yocto, ContractAccount, UserAccount,
};

lazy_static_include::lazy_static_include_bytes! {
   NEAR_CAMPAIGN => "wasm/near_campaign.wasm"
}

pub fn init() {
  let root = init_simulator(None);

  let contract = deploy! {
        contract: Campaign,
        contract_id: "campaign1",
        bytes: &NEAR_CAMPAIGN,
        signer_account: root,
    // account_id: "someid"
        init_method: new(to_yocto("1"))

  };
}

// let near_campaign = root.deploy(&NEAR_CAMPAIGN, "campaign1".parse().unwrap(), to_yocto("10"));
