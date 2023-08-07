use soroban_sdk::{Env, String};

use crate::{ExternalDataProvider, ExternalDataProviderClient};

#[test]
pub fn test() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let reputation = external_data_provider_client
    .get_reputation_category_for_user(&String::from_slice(&env, "user001"));
  assert!(reputation.is_none());

  external_data_provider_client.mock_sample_data();

  let reputation = external_data_provider_client
    .get_reputation_category_for_user(&String::from_slice(&env, "user001"));
  assert!(reputation.unwrap() == 0);
  let reputation = external_data_provider_client
    .get_reputation_category_for_user(&String::from_slice(&env, "user002"));
  assert!(reputation.unwrap() == 1);
}
