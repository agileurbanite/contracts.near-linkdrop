use near_sdk_sim::{lazy_static_include, to_yocto, UserAccount};

/*
  external_linkdrop.wasm is an original linkdrop contract - https://github.com/near/near-linkdrop.
  We use this contract as an account creator for new users.
*/

lazy_static_include::lazy_static_include_bytes! {
   EXTERNAL_LINKDROP => "wasm/external_linkdrop.wasm"
}

pub fn init_external_linkdrop(root: &UserAccount) {
  root.deploy(
    &EXTERNAL_LINKDROP,
    "testnet".parse().unwrap(),
    to_yocto("5"),
  );
}
