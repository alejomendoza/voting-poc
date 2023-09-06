use soroban_sdk::{vec, Env, String, Vec};

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
pub fn test_delegation() {
  let env = Env::default();

  let external_data_provider_id = env.register_contract(None, ExternalDataProvider);
  let external_data_provider_client =
    ExternalDataProviderClient::new(&env, &external_data_provider_id);

  let user_id_1 = String::from_slice(&env, "user001");
  let user_id_2 = String::from_slice(&env, "user002");
  let user_id_3 = String::from_slice(&env, "user002");

  assert!(external_data_provider_client.get_delegatees().is_empty());

  external_data_provider_client.mock_sample_data();

  assert!(external_data_provider_client.get_delegatees().len() == 1);
  assert!(
    external_data_provider_client
      .get_delegatees()
      .get(user_id_1)
      .unwrap()
      .len()
      == 6
  );
  assert!(external_data_provider_client
    .get_delegatees()
    .get(user_id_2.clone())
    .is_none());
  external_data_provider_client.set_delegatees_for_user(
    &user_id_2,
    &Vec::from_slice(
      &env,
      &[
        String::from_slice(&env, "user011"),
        String::from_slice(&env, "user012"),
        String::from_slice(&env, "user013"),
        String::from_slice(&env, "user014"),
        String::from_slice(&env, "user015"),
      ],
    ),
  );

  // exceeed limits
  assert!(external_data_provider_client
    .try_set_delegatees_for_user(
      &user_id_3,
      &Vec::from_slice(
        &env,
        &[
          String::from_slice(&env, "user021"),
          String::from_slice(&env, "user022"),
          String::from_slice(&env, "user023"),
          String::from_slice(&env, "user024"),
          String::from_slice(&env, "user025"),
          String::from_slice(&env, "user026"),
          String::from_slice(&env, "user027"),
          String::from_slice(&env, "user028"),
          String::from_slice(&env, "user029"),
          String::from_slice(&env, "user030"),
          String::from_slice(&env, "user031"),
        ],
      ),
    )
    .is_err());
  assert!(external_data_provider_client
    .try_set_delegatees_for_user(
      &user_id_3,
      &Vec::from_slice(
        &env,
        &[
          String::from_slice(&env, "user021"),
          String::from_slice(&env, "user022"),
          String::from_slice(&env, "user023"),
          String::from_slice(&env, "user024"),
        ],
      ),
    )
    .is_err());

  assert!(
    external_data_provider_client
      .get_delegatees()
      .get(user_id_2.clone())
      .unwrap()
      .len()
      == 5
  );
  assert!(external_data_provider_client.get_delegatees().len() == 2);
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
}
