use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{vec, Env, Map, String, Vec};

use crate::{ExternalDataProvider, ExternalDataProviderClient, ReputationCategory};

fn initialize_external_data_provider(env: &Env) -> ExternalDataProviderClient {
  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  external_data_provider_client
}

#[test]
pub fn test_reputation() {
  let env = Env::default();

  let external_data_provider_client = initialize_external_data_provider(&env);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");

  let reputation = external_data_provider_client.get_user_reputation_category(&user_id_1);
  assert!(reputation == ReputationCategory::Uncategorized);

  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::Uncategorized)
      == (0, 0)
  );
  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::Poor) == (0, 100)
  );
  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::Average) == (0, 100)
  );
  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::Good) == (0, 200)
  );
  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::VeryGood) == (0, 200)
  );
  assert!(
    external_data_provider_client.get_reputation_score(&ReputationCategory::Excellent) == (0, 300)
  );

  let reputation_scores = external_data_provider_client.get_reputation_scores();
  assert!(reputation_scores.get(0).unwrap() == (0, 0));
  assert!(reputation_scores.get(1).unwrap() == (0, 100));
  assert!(reputation_scores.get(2).unwrap() == (0, 100));
  assert!(reputation_scores.get(3).unwrap() == (0, 200));
  assert!(reputation_scores.get(4).unwrap() == (0, 200));
  assert!(reputation_scores.get(5).unwrap() == (0, 300));

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
  external_data_provider_client.set_users_rep_categories(&users_reputation_categories);

  assert!(
    external_data_provider_client.get_user_reputation_category(&user_id_1)
      == ReputationCategory::Poor
  );
  assert!(
    external_data_provider_client.get_user_reputation_category(&user_id_2)
      == ReputationCategory::Good
  );

  // test set_users_rep_categories_vec
  let mut users_reputation_categories_vec: Vec<(String, String)> = Vec::new(&env);
  for (user_id, rep_category) in users_reputation_categories {
    users_reputation_categories_vec.push_back((user_id.clone(), rep_category.clone()));
  }
  external_data_provider_client.set_users_rep_categories_vec(&users_reputation_categories_vec);
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

  let external_data_provider_client = initialize_external_data_provider(&env);

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
  let fetched_bonus_map = external_data_provider_client.get_round_bonus_map();
  assert!(fetched_bonus_map.len() == 5);

  // test set_round_bonus_map_vec
  let mut round_bonus_map_vec: Vec<(u32, u32)> = Vec::new(&env);
  for (round, bonus) in round_bonus_map {
    round_bonus_map_vec.push_back((round.clone(), DecimalNumberWrapper::from(bonus).as_raw()));
  }
  external_data_provider_client.set_round_bonus_map_vec(&round_bonus_map_vec);
  let fetched_bonus_map2 = external_data_provider_client.get_round_bonus_map();
  assert!(fetched_bonus_map == fetched_bonus_map2);

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

  let external_data_provider_client = initialize_external_data_provider(&env);

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
  external_data_provider_client.set_users_delegation_ranks(&users_ranks);

  let ranks = external_data_provider_client.get_delegation_ranks_for_users(&vec![
    &env,
    user_id_1.clone(),
    user_id_99.clone(),
  ]);
  assert!(ranks.len() == 2);
  assert!(ranks.get(user_id_1.clone()).unwrap() == 3);
  assert!(ranks.get(user_id_99.clone()).unwrap() == 6);

  // test set_users_delegation_ranks_vec
  let mut users_ranks_vec: Vec<(String, u32)> = Vec::new(&env);
  for (user_id, rank) in users_ranks {
    users_ranks_vec.push_back((user_id.clone(), rank.clone()));
  }
  external_data_provider_client.set_users_delegation_ranks_vec(&users_ranks_vec);
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

  let external_data_provider_client = initialize_external_data_provider(&env);

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

  let external_data_provider_client = initialize_external_data_provider(&env);

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
pub fn test_page_rank() {
  let env = Env::default();

  let external_data_provider_client = initialize_external_data_provider(&env);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");
  let user_id_3 = String::from_slice(&env, "user003");
  let user_id_4 = String::from_slice(&env, "user004");

  let mut new_trust_map: Map<String, Map<String, ()>> = Map::new(&env);
  new_trust_map.set(
    user_id_1.clone(),
    Map::from_array(&env, [(user_id_2.clone(), ())]),
  );
  new_trust_map.set(
    user_id_2.clone(),
    Map::from_array(&env, [(user_id_3.clone(), ()), (user_id_4.clone(), ())]),
  );
  new_trust_map.set(
    user_id_3.clone(),
    Map::from_array(&env, [(user_id_1.clone(), ()), (user_id_4.clone(), ())]),
  );
  new_trust_map.set(
    user_id_4.clone(),
    Map::from_array(&env, [(user_id_2.clone(), ())]),
  );
  external_data_provider_client.set_trust_map(&new_trust_map);

  external_data_provider_client.calculate_page_rank();
  let results = external_data_provider_client.get_page_rank_results();

  assert!(results.get(user_id_1.clone()).unwrap() == (0, 177));
  assert!(results.get(user_id_2.clone()).unwrap() == (0, 331));
  assert!(results.get(user_id_3.clone()).unwrap() == (0, 337));
  assert!(results.get(user_id_4.clone()).unwrap() == (0, 177));

  assert!(external_data_provider_client.get_page_rank_result_for_user(&user_id_1) == (0, 177));

  external_data_provider_client.set_page_rank_result(&Map::from_array(
    &env,
    [
      (user_id_1.clone(), (0, 100)),
      (user_id_2.clone(), (0, 200)),
      (user_id_3.clone(), (0, 300)),
      (user_id_4.clone(), (0, 400)),
    ],
  ));

  assert!(
    external_data_provider_client
      .get_page_rank_results()
      .get(user_id_1.clone())
      .unwrap()
      == (0, 100)
  );
  assert!(external_data_provider_client.get_page_rank_result_for_user(&user_id_2) == (0, 200));
}

#[test]
pub fn test_set_trust_map_for_user() {
  let env = Env::default();

  let external_data_provider_client = initialize_external_data_provider(&env);

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
  assert!(user_2_map.is_none());

  // test set_trust_map_for_user_vec
  external_data_provider_client
    .set_trust_map_for_user_vec(&user_id_1, &Vec::from_array(&env, [user_id_2.clone()]));

  let trust_map = external_data_provider_client.get_trust_map();
  let user_1_map = trust_map.get(user_id_1.clone());
  let user_2_map = trust_map.get(user_id_2.clone());

  assert!(user_1_map == Some(Map::from_array(&env, [(user_id_2.clone(), ()),])));
  assert!(user_2_map.is_none());
}
