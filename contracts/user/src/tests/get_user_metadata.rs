use super::utils::{create_user, get_context};
use near_sdk::testing_env;

#[test]
fn get_user_metadata() {
    let contract = create_user();
    testing_env!(get_context());

    let result = contract.get_user_metadata();

    assert_eq!("1.0".to_string(), result.version);
}
