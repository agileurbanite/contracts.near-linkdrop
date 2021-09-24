use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Cannot refund inactive or non-existing key"#)]
fn refund_inactive_keys() {
    let keys = keys::get_public_keys(0, 1);
    let some_keys = keys::get_public_keys(0, 0);
    let mut context = get_context();
    context.account_balance = 2_000_000_000_000_000_000_000_000;
    testing_env!(context);

    let mut contract = create_campaign();
    // It is planned to add 2 keys
    contract.keys_stats.total = 2;
    contract.add_keys(keys);

    contract.refund_keys(
        some_keys.clone(),
        "b.testnet".parse().unwrap()
    );

    // Retry using the same key
    contract.refund_keys(
        some_keys,
        "b.testnet".parse().unwrap()
    );
}
