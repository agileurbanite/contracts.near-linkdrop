use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Cannot refund inactive or non-existing key"#)]
fn refund_non_existing_keys() {
    let keys = keys::get_public_keys(0, 0);
    let other_keys = keys::get_public_keys(1, 1);
    let mut context = get_context();
    context.account_balance = 2_000_000_000_000_000_000_000_000;
    testing_env!(context);

    let mut contract = create_campaign();
    contract.add_keys(keys);

    // Try using other keys
    contract.refund_keys(
        other_keys,
        "b.testnet".parse().unwrap()
    );
}
