use super::utils::{create_campaign, get_context, keys};
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Cannot claim by inactive or non-existing key"#)]
fn claim_non_existing_keys() {
    let keys = keys::get_public_keys(0, 0);
    let other_keys = keys::get_public_keys(1, 1);
    let mut context = get_context();

    context.signer_account_id = "b.testnet".parse().unwrap();
    context.predecessor_account_id = "b.testnet".parse().unwrap();
    context.signer_account_pk = other_keys[0].clone().into();
    context.account_balance = 1_000_000_000_000_000_000_000_000;

    testing_env!(context);

    let mut contract = create_campaign();
    contract.add_keys(keys);

    // Try using other keys
    contract.claim("c.testnet".parse().unwrap());
}
