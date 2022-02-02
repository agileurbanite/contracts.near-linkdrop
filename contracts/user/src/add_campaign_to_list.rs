use crate::*;

#[near_bindgen]
impl User {
  #[private]
  pub fn add_campaign_to_list(&mut self, campaign_id: AccountId) {
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
