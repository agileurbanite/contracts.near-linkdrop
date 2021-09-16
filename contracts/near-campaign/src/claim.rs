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
    // TODO do we need to check that the key is the functional call access key?
    // Right now it is possible to call this method with full-access key and it will cause
    // we won't be able to delete the account and return tokens back to the owner.

    let key = env::signer_account_pk();

    self.keys.insert(&key, &KeyStatus::Claimed);
    self.keys_stats.active -= 1;
    self.keys_stats.claimed += 1;

    if self.keys_stats.active == 0 {
      self.status = CampaignStatus::Completed;
    };

    Promise::new(env::current_account_id())
      .delete_key(key)
      .then(Promise::new(account_id).transfer(self.tokens_per_key))
  }
}
