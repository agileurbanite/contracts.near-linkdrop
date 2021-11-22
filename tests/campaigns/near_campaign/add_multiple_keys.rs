use crate::utils::NearCampaignUtility;
use near_sdk_sim::{call, to_yocto, DEFAULT_GAS};

#[test]
fn add_multiple_keys() {
  let number_of_keys: u64 = 200;
  let tera_gas = u64::pow(10, 12);

  let expected_gas_by_adding_one_key: u64 = tera_gas * 4; // 4 TeraGas
  let expected_gas_ceiling: u64 = tera_gas * 200; // 200 TeraGas
  let expected_storage_usage_per_one_key: u64 = 390; // bytes

  let near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    1,
    "5",
    0,
    0
  );
  let contract = &near_campaign_utility.contract;
  let keys = &near_campaign_utility.keys;

  // Add one key
  let storage_usage_before_adding_one_key = near_campaign_utility.get_contract_storage_usage();

  let mut result = call!(
    contract.user_account,
    contract.add_keys(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  let storage_usage_after_adding_one_key = near_campaign_utility.get_contract_storage_usage();
  let storage_usage_per_one_key =
    storage_usage_after_adding_one_key - storage_usage_before_adding_one_key;

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

  // Re-initialization of the environment
  let near_campaign_utility = NearCampaignUtility::init_near_campaign(
    to_yocto("200"),
    number_of_keys,
    "5",
    0,
    number_of_keys as usize - 1
  );
  let contract = &near_campaign_utility.contract;
  let keys = &near_campaign_utility.keys;

  // Add 200 keys
  let storage_usage_at_begin = near_campaign_utility.get_contract_storage_usage();

  result = call!(
    contract.user_account,
    contract.add_keys(keys.public_keys()),
    gas = DEFAULT_GAS
  );
  result.assert_success();

  let storage_usage_at_end = near_campaign_utility.get_contract_storage_usage();
  let storage_usage_total = storage_usage_at_end - storage_usage_at_begin;

  println!("\nadd_multiple_keys > Number of keys: {}\n", number_of_keys);
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
  assert_eq!(
    storage_usage_total,
    storage_usage_per_one_key * number_of_keys
  )
}
