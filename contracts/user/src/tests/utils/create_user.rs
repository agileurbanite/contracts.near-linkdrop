use crate::*;

pub fn create_user() -> User {
    let mut user = User::new();
    user.future_campaign_id += 1;
    user.campaigns.insert(&"campaign1.user.testnet".parse().unwrap());
    user.future_campaign_id += 1;
    user.campaigns.insert(&"campaign2.user.testnet".parse().unwrap());
    // Two test companies were created
    user
}
