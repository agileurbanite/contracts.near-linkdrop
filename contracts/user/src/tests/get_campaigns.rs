use super::utils::{create_user, get_context};
use near_sdk::testing_env;

#[test]
fn get_campaigns() {
    let contract = create_user();
    testing_env!(get_context());

    let result = contract.get_campaigns();

    assert_eq!(2, result.len());
    assert_eq!("campaign1.user.testnet", result[0].as_str());
    assert_eq!("campaign2.user.testnet", result[1].as_str());
}
