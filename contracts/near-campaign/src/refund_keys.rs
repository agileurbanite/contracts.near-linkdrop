use crate::gas::*;
use crate::*;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn refund_keys(&mut self, keys: Vec<PublicKey>, beneficiary_id: AccountId) {
    assert_eq!(
      self.status,
      CampaignStatus::Active,
      "Unable to call this method on inactive campaign"
    );

    keys.into_iter().for_each(|pk| {
      let key = pk.clone().into();

      assert_eq!(
        self.keys.get(&key),
        Some(KeyStatus::Active),
        "Cannot refund inactive or non-existing key"
      );

      Promise::new(beneficiary_id.clone())
        .transfer(self.tokens_per_key)
        .then(ext_self::on_refunded_key(
          key,
          env::current_account_id(),
          0,
          BASE_GAS, // 25 TGas
        ));
    });
  }
}
