use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Key is already exists"#)]
fn add_keys_is_already_exists() {
    testing_env!(get_context());
    let mut contract = create_campaign();
    // It is planned to add 2 keys
    contract.keys_stats.total = 2;
    let keys = keys::get_public_keys(0, 0);

    // Add the first key
    contract.add_keys(keys.clone());

    // Let's try to add the same key again
    contract.add_keys(keys);
}
