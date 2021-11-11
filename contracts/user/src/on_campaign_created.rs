use crate::*;

#[near_bindgen]
impl User {
  pub fn on_campaign_created(&mut self, campaign_id: AccountId) {
    // TODO only campaign creator can call this method - add validation
    self.campaigns.insert(&campaign_id);

    env::log_str(
      format!(
        "Campaign @{} was added into @{} state",
        campaign_id,
        env::current_account_id()
      )
      .as_str(),
    );
  }
}
