use soroban_sdk::{Env, String};
use voting_shared::types::ReputationCategory;

use crate::{ExternalDataProvider, ExternalDataProviderClient};

#[test]
pub fn test() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let reputation = external_data_provider_client
    .get_user_reputation_category(&String::from_slice(&env, "user001"));
  assert!(reputation == ReputationCategory::Uncategorized);

  external_data_provider_client.mock_sample_data();

  let reputation = external_data_provider_client
    .get_user_reputation_category(&String::from_slice(&env, "user001"));
  assert!(reputation == ReputationCategory::Excellent);
  let reputation = external_data_provider_client
    .get_user_reputation_category(&String::from_slice(&env, "user002"));
  assert!(reputation == ReputationCategory::VeryGood);
}
