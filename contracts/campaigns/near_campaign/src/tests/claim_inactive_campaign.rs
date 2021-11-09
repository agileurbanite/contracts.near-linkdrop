use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Unable to call this method on inactive campaign"#)]
fn claim_inactive_campaign() {
    let keys = keys::get_public_keys(0, 0);
    testing_env!(get_context());

    let mut contract = create_campaign();
    contract.add_keys(keys);

    // Before that all the keys were used
    contract.status = CampaignStatus::Completed;

    contract.claim("c.testnet".parse().unwrap());
}
