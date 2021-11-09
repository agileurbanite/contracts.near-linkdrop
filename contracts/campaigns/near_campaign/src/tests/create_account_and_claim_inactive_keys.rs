use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
#[should_panic(expected = r#"Cannot create account by inactive or non-existing key"#)]
fn create_account_and_claim_inactive_keys() {
    let keys = keys::get_public_keys(0, 1);
    let new_keys = keys::get_public_keys(2, 3);
    let mut context = get_context();

    context.signer_account_id = "b.testnet".parse().unwrap();
    context.predecessor_account_id = "b.testnet".parse().unwrap();
    context.signer_account_pk = keys[0].clone().into();
    context.account_balance = 1_000_000_000_000_000_000_000_000;

    testing_env!(context);

    let mut contract = create_campaign();
    // It is planned to add 2 keys
    contract.keys_stats.total = 2;
    contract.add_keys(keys.clone());

    // Create account and claim one link
    contract.keys.insert(&keys[0].clone().into(), &KeyStatus::Created);
    contract.keys_stats.active -= 1;
    contract.keys_stats.created += 1;

    // Retry using the same key
    contract.create_account_and_claim(
        "d.testnet".parse().unwrap(),
        new_keys[1].clone()
    );
}
