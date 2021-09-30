use near_sdk::Gas;

pub const BASE_GAS: Gas = Gas(25_000_000_000_000); // 25 TGas

pub fn t_gas(value: u64) -> Gas {
  Gas(value * 10u64.pow(12))
}
