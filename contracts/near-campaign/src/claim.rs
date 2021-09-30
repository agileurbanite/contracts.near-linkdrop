use crate::gas::*;
use crate::*;

/*
 'claim' allows us to send some amount of NEAR that is defined by 'tokens_per_key' to any account
 This method has the same signature as the original contract - so a user can use NEAR Wallet
 to claim his tokens.
 https://github.com/near/near-linkdrop/blob/master/src/lib.rs#L72
*/

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn claim(&mut self, account_id: AccountId) -> Promise {
    assert_eq!(
      self.status,
      CampaignStatus::Active,
      "Unable to call this method on inactive campaign"
    );
    assert_eq!(
      self.keys.get(&env::signer_account_pk()),
      Some(KeyStatus::Active),
      "Cannot claim by inactive or non-existing key"
    );

    Promise::new(account_id)
      .transfer(self.tokens_per_key)
      .then(ext_self::on_claimed(
        env::current_account_id(),
        0,
        BASE_GAS, // 25 TGas
      ))
  }
}
