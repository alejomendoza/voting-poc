use soroban_sdk::{vec, Env, Map, String, Vec};

use crate::{ExternalDataProvider, ExternalDataProviderClient, ReputationCategory};

#[test]
pub fn test_reputation() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_1);
  assert!(reputation == ReputationCategory::Uncategorized);

  external_data_provider_client.mock_sample_data();

  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_1);
  assert!(reputation == ReputationCategory::Excellent);
  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_2);
  assert!(reputation == ReputationCategory::VeryGood);

  let mut reputation_categories = external_data_provider_client.get_reputation_categories();
  reputation_categories.set(user_id_1.clone(), ReputationCategory::Poor);
  external_data_provider_client.set_user_reputation_categories(&reputation_categories);

  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_1);
  assert!(reputation == ReputationCategory::Poor);

  external_data_provider_client
    .set_user_reputation_category(&user_id_1, &String::from_slice(&env, "Good"));
  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_1);
  assert!(reputation == ReputationCategory::Good);

  let mut users_reputation_categories: Map<String, String> = Map::new(&env);
  users_reputation_categories.set(user_id_1.clone(), String::from_slice(&env, "Poor"));
  users_reputation_categories.set(user_id_2.clone(), String::from_slice(&env, "Good"));
  external_data_provider_client.set_users_reputation_categories(&users_reputation_categories);

  assert!(
    external_data_provider_client.get_user_reputation_category(&user_id_1)
      == ReputationCategory::Poor
  );
  assert!(
    external_data_provider_client.get_user_reputation_category(&user_id_2)
      == ReputationCategory::Good
  );
}

#[test]
pub fn test_history() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  assert!(external_data_provider_client
    .get_user_prior_voting_history(&user_id_1)
    .is_empty());
  assert!(external_data_provider_client
    .get_round_bonus_map()
    .is_empty());

  external_data_provider_client.mock_sample_data();

  let mut round_bonus_map = external_data_provider_client.get_round_bonus_map();
  assert!(round_bonus_map.len() == 4);
  round_bonus_map.set(5, (0, 400));
  external_data_provider_client.set_round_bonus_map(&round_bonus_map);
  assert!(external_data_provider_client.get_round_bonus_map().len() == 5);

  assert!(
    external_data_provider_client
      .get_prior_voting_history()
      .len()
      == 2
  );
  external_data_provider_client
    .set_user_prior_voting_history(&user_id_2, &Vec::from_slice(&env.clone(), &[1, 3, 4]));
  assert!(
    external_data_provider_client
      .get_prior_voting_history()
      .len()
      == 3
  );

  assert!(
    external_data_provider_client.get_user_prior_voting_history(&user_id_1)
      == Vec::from_slice(&env, &[2, 3])
  );
  external_data_provider_client
    .set_user_prior_voting_history(&user_id_1, &Vec::from_slice(&env, &[3, 4]));
  assert!(
    external_data_provider_client.get_user_prior_voting_history(&user_id_1)
      == Vec::from_slice(&env, &[3, 4])
  );
}

#[test]
pub fn test_delegation_rank() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");
  let user_id_3 = String::from_slice(&env, "user003");
  let user_id_99 = String::from_slice(&env, "user099");

  assert!(external_data_provider_client
    .get_delegation_ranks()
    .is_empty());

  external_data_provider_client.mock_sample_data();

  assert!(external_data_provider_client.get_delegation_ranks().len() == 8);

  let ranks = external_data_provider_client.get_delegation_ranks_for_users(&vec![
    &env,
    user_id_1.clone(),
    user_id_2.clone(),
    user_id_3.clone(),
  ]);
  assert!(ranks.len() == 3);
  assert!(ranks.get(user_id_1.clone()).unwrap() == 1);
  assert!(ranks.get(user_id_2.clone()).unwrap() == 2);
  assert!(ranks.get(user_id_3.clone()).unwrap() == 3);

  external_data_provider_client.set_delegation_rank_for_user(&user_id_1.clone(), &15);

  let ranks = external_data_provider_client.get_delegation_ranks_for_users(&vec![
    &env,
    user_id_1.clone(),
    user_id_99.clone(),
  ]);
  assert!(ranks.len() == 2);
  assert!(ranks.get(user_id_1.clone()).unwrap() == 15);
  assert!(ranks.get(user_id_99.clone()).unwrap() == 0);

  let mut users_ranks: Map<String, u32> = Map::new(&env);
  users_ranks.set(user_id_1.clone(), 3);
  users_ranks.set(user_id_99.clone(), 6);
  external_data_provider_client.set_delegation_ranks_for_users(&users_ranks);

  let ranks = external_data_provider_client.get_delegation_ranks_for_users(&vec![
    &env,
    user_id_1.clone(),
    user_id_99.clone(),
  ]);
  assert!(ranks.len() == 2);
  assert!(ranks.get(user_id_1.clone()).unwrap() == 3);
  assert!(ranks.get(user_id_99.clone()).unwrap() == 6);
}

#[test]
pub fn test_trust_map() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  assert!(external_data_provider_client
    .get_trust_map()
    .get(user_id_1.clone())
    .is_none());

  external_data_provider_client.mock_sample_data();

  let trust_map = external_data_provider_client.get_trust_map();
  let user_1_map = trust_map.get(user_id_1.clone());
  let user_2_map = trust_map.get(user_id_2.clone());
  assert!(
    user_1_map
      == Some(Map::from_array(
        &env,
        [
          (user_id_2.clone(), ()),
          (String::from_slice(&env, "user004"), ())
        ]
      ))
  );
  assert!(user_2_map == Some(Map::from_array(&env, [(user_id_1.clone(), ()),])));
}

#[test]
pub fn test_set_trust_map() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  let mut new_trust_map: Map<String, Map<String, ()>> = Map::new(&env);
  new_trust_map.set(
    user_id_1.clone(),
    Map::from_array(&env, [(user_id_2.clone(), ())]),
  );
  external_data_provider_client.set_trust_map(&new_trust_map);

  let trust_map = external_data_provider_client.get_trust_map();
  let user_1_map = trust_map.get(user_id_1.clone());
  let user_2_map = trust_map.get(user_id_2.clone());

  assert!(user_1_map == Some(Map::from_array(&env, [(user_id_2.clone(), ()),])));
  assert!(user_2_map.is_none())
}

#[test]
pub fn test_set_trust_map_for_user() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  external_data_provider_client.set_trust_map_for_user(
    &user_id_1,
    &Map::from_array(&env, [(user_id_2.clone(), ())]),
  );

  let trust_map = external_data_provider_client.get_trust_map();
  let user_1_map = trust_map.get(user_id_1.clone());
  let user_2_map = trust_map.get(user_id_2.clone());

  assert!(user_1_map == Some(Map::from_array(&env, [(user_id_2.clone(), ()),])));
  assert!(user_2_map.is_none())
}
