use near_sdk_sim::{lazy_static_include, to_yocto, UserAccount};

lazy_static_include::lazy_static_include_bytes! {
   EXTERNAL_LINKDROP => "wasm/external_linkdrop.wasm"
}

pub fn init_external_linkdrop(root: &UserAccount) {
  root.deploy(&EXTERNAL_LINKDROP, "testnet".to_string(), to_yocto("5"));
}
