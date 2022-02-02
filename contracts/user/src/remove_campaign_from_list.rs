use crate::*;

#[near_bindgen]
impl User {
  #[private]
  pub fn remove_campaign_from_list(&mut self, campaign_id: AccountId) {
    self.campaigns.remove(&campaign_id);

    env::log_str(
      format!(
        "Campaign @{} was removed from @{} state",
        campaign_id,
        env::current_account_id()
      )
      .as_str(),
    );
  }
}
