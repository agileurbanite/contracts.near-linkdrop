use near_sdk_sim::to_yocto;

pub fn assert_almost_eq_with_max_delta(left: u128, right: u128, max_delta: u128) {
  assert!(
    std::cmp::max(left, right) - std::cmp::min(left, right) <= max_delta,
    "{}",
    format!(
      "Left {} is not even close to Right {} within delta {}",
      left, right, max_delta
    )
  );
}

pub fn assert_eq_with_gas(left: u128, right: u128) {
  assert_almost_eq_with_max_delta(left, right, to_yocto("0.006"));
}
