use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Unable to call this method on inactive campaign"#)]
fn create_account_and_claim_inactive_campaign() {
    let keys = keys::get_public_keys(0, 0);
    let new_keys = keys::get_public_keys(1, 1);
    testing_env!(get_context());

    let mut contract = create_campaign();
    contract.add_keys(keys);

    // Before that all the keys were used
    contract.status = CampaignStatus::Completed;

    // The company no longer has an active status
    contract.create_account_and_claim(
        "c.testnet".parse().unwrap(),
        new_keys[0].clone()
    );
}
