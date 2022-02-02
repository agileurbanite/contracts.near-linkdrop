use near_sdk::Gas;

pub fn tgas(value: u64) -> Gas {
  Gas(value * 10u64.pow(12))
}
