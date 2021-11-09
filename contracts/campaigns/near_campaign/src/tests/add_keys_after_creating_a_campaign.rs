use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Unable to call this method after creating a campaign"#)]
fn add_keys_after_creating_a_campaign() {
    testing_env!(get_context());
    let mut contract = create_campaign();
    let keys = keys::get_public_keys(0, 0);

    // Only one key needs to be added
    contract.add_keys(keys);
    // The status of the company has changed to 'Active'
    assert_eq!(CampaignStatus::Active, contract.status);

    // New keys cannot be added
    contract.add_keys(keys::get_public_keys(1, 1));
}
