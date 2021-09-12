use crate::utils::{init_near_campaign, KeySet};
use near_sdk_sim::{call, DEFAULT_GAS, UserAccount, ContractAccount};

fn get_contract_storage_usage<T>(root_user: &UserAccount, contract: &ContractAccount<T>) -> u64 {
  let runtime = root_user.borrow_runtime();
  runtime.view_account(
    contract.account_id().as_str()
  ).unwrap().storage_usage
}

#[test]
fn add_multiple_keys() {
  let number_of_keys: u64 = 200;
  let tera_gas = u64::pow(10, 12);

  let expected_gas_by_adding_one_key: u64 = tera_gas * 4; // 4 TeraGas
  let expected_gas_ceiling: u64 = tera_gas * 180; // 180 TeraGas
  let expected_storage_usage_per_one_key: u64 = 390; // bytes

  let (root, near_campaign) = init_near_campaign("5");

  // Add one key
  let storage_usage_before_adding_one_key = get_contract_storage_usage(
    &root,
    &near_campaign
  );

  let mut key_set = KeySet::create(0, 0);
  let mut result = call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  let storage_usage_after_adding_one_key = get_contract_storage_usage(
    &root,
    &near_campaign
  );
  let storage_usage_per_one_key =
    storage_usage_after_adding_one_key - storage_usage_before_adding_one_key;

  {
    println!(
      "\nadd_multiple_keys > TeraGas burnt by adding one key: {}",
      result.gas_burnt().0 as f64 / 1e12
    );
    println!(
      "add_multiple_keys > Tokens burnt by adding one key: {}",
      result.tokens_burnt() as f64 / 1e24
    );
    assert!(result.gas_burnt().0 < expected_gas_by_adding_one_key);

    println!(
      "add_multiple_keys > Storage usage per one key: {}",
      storage_usage_per_one_key
    );
    assert!(storage_usage_per_one_key < expected_storage_usage_per_one_key);
  }

  // Re-initialization of the environment
  let (root, near_campaign) = init_near_campaign("5");

  // Add 200 keys
  let storage_usage_at_begin = get_contract_storage_usage(
    &root,
    &near_campaign
  );

  key_set = KeySet::create(0, number_of_keys as usize - 1);
  result = call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  let storage_usage_at_end = get_contract_storage_usage(
    &root,
    &near_campaign
  );
  let storage_usage_total = storage_usage_at_end - storage_usage_at_begin;

  {
    println!(
      "\nadd_multiple_keys > Number of keys: {}\n",
      number_of_keys
    );
    println!(
      "add_multiple_keys > Total TeraGas burnt: {}",
      result.gas_burnt().0 as f64 / 1e12
    );
    println!(
      "add_multiple_keys > Total tokens burnt: {}",
      result.tokens_burnt() as f64 / 1e24
    );
    assert!(result.gas_burnt().0 < expected_gas_ceiling);

    println!(
      "add_multiple_keys > Total storage usage: {}",
      storage_usage_total
    );
    assert_eq!(storage_usage_total, storage_usage_per_one_key * number_of_keys)
  }
}
