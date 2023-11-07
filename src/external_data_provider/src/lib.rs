#![no_std]
#![allow(non_upper_case_globals)]

mod page_rank;
pub mod types;

use page_rank::Rank;

// This contract's going to be responsible for fetching the data from any external resources

use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, Map, String, Vec};
use types::{reputation_category_from_str, ReputationCategory};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  // storage type: instance
  // Map<UserUUID, ReputationCategory> - users to their categories
  Reputation,
  // storage type: instance
  // Map<UserUUID, Vec<u32>> - users to the vector of rounds they participated in
  PriorVotingHistory,
  // storage type: instance
  // Map<u32, DecimalNumber> - (connected to PRIOR_VOTING_HISTORY) rounds to their bonus (for participation)
  RoundBonusMap,
  // storage type: instance
  // Map<UserUUID, u32> - users to their delegation rank
  DelegationRanks,
  // storage type: instance
  // Map<String, Map<String, ()>> - users to their delegation rank
  TrustMap,
  // storage type: instance
  // (u32, u32)
  PageRankResult,
}

#[contract]
pub struct ExternalDataProvider;

impl ExternalDataProvider {}

#[contractimpl]
impl ExternalDataProvider {
  fn generate_username(env: &Env, n: u32) -> String {
    match n {
      1 => String::from_slice(&env, "user001"),
      2 => String::from_slice(&env, "user002"),
      3 => String::from_slice(&env, "user003"),
      4 => String::from_slice(&env, "user004"),
      5 => String::from_slice(&env, "user005"),
      6 => String::from_slice(&env, "user006"),
      7 => String::from_slice(&env, "user007"),
      8 => String::from_slice(&env, "user008"),
      9 => String::from_slice(&env, "user009"),
      _ => String::from_slice(&env, "userXXX"),
    }
  }

  // done
  pub fn mock_data_assigned_reputation(env: Env) {
    let mut reputation_map: Map<String, ReputationCategory> = Map::new(&env);
    reputation_map.set(
      ExternalDataProvider::generate_username(&env, 1),
      ReputationCategory::Excellent,
    );
    reputation_map.set(
      ExternalDataProvider::generate_username(&env, 2),
      ReputationCategory::VeryGood,
    );
    reputation_map.set(
      ExternalDataProvider::generate_username(&env, 3),
      ReputationCategory::Good,
    );
    reputation_map.set(
      ExternalDataProvider::generate_username(&env, 4),
      ReputationCategory::Average,
    );
    reputation_map.set(
      ExternalDataProvider::generate_username(&env, 5),
      ReputationCategory::Poor,
    );
    env
      .storage()
      .instance()
      .set(&DataKey::Reputation, &reputation_map);
  }

  pub fn mock_prior_voting_history(env: Env) {
    // for prior history neuron
    let mut voting_history_set: Map<String, Vec<u32>> = Map::new(&env);
    voting_history_set.set(
      ExternalDataProvider::generate_username(&env, 1),
      vec![&env, 2, 3],
    );
    voting_history_set.set(
      ExternalDataProvider::generate_username(&env, 3),
      vec![&env, 2, 3, 4],
    );
    env
      .storage()
      .instance()
      .set(&DataKey::PriorVotingHistory, &voting_history_set);
  }

  pub fn mock_round_bonus_map(env: Env) {
    let mut round_bonus_map: Map<u32, (u32, u32)> = Map::new(&env);
    round_bonus_map.set(1, (0, 0));
    round_bonus_map.set(2, (0, 100));
    round_bonus_map.set(3, (0, 200));
    round_bonus_map.set(4, (0, 300));
    env
      .storage()
      .instance()
      .set(&DataKey::RoundBonusMap, &round_bonus_map);
  }

  // done
  pub fn mock_delegation_ranks(env: Env) {
    let mut delegation_ranks: Map<String, u32> = Map::new(&env);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 1), 1);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 2), 2);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 3), 3);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 4), 4);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 5), 5);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 6), 6);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 7), 7);
    delegation_ranks.set(ExternalDataProvider::generate_username(&env, 8), 8);
    env
      .storage()
      .instance()
      .set(&DataKey::DelegationRanks, &delegation_ranks);
  }

  pub fn mock_trust_map(env: Env) {
    let user001 = ExternalDataProvider::generate_username(&env, 1);
    let user002 = ExternalDataProvider::generate_username(&env, 2);
    let user003 = ExternalDataProvider::generate_username(&env, 3);
    let user004 = ExternalDataProvider::generate_username(&env, 4);
    // for trust graph neuron
    let mut trust_map: Map<String, Map<String, ()>> = Map::new(&env);
    trust_map.set(
      user001.clone(),
      // map[&env, user002.clone(), user004.clone()],
      Map::from_array(&env, [(user002.clone(), ()), (user004.clone(), ())]),
    );
    trust_map.set(
      user002.clone(),
      Map::from_array(&env, [(user001.clone(), ())]),
    );
    trust_map.set(
      user003.clone(),
      Map::from_array(&env, [(user001.clone(), ()), (user002.clone(), ())]),
    );
    trust_map.set(
      user004.clone(),
      Map::from_array(&env, [(user003.clone(), ())]),
    );
    env.storage().instance().set(&DataKey::TrustMap, &trust_map);
  }

  pub fn mock_sample_data(env: Env) {
    ExternalDataProvider::mock_data_assigned_reputation(env.clone());
    ExternalDataProvider::mock_prior_voting_history(env.clone());
    ExternalDataProvider::mock_round_bonus_map(env.clone());
    ExternalDataProvider::mock_delegation_ranks(env.clone());
    ExternalDataProvider::mock_trust_map(env.clone());
  }

  // for assigned reputation neuron
  pub fn get_reputation_categories(env: Env) -> Map<String, ReputationCategory> {
    env
      .storage()
      .instance()
      .get(&DataKey::Reputation)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_user_reputation_category(env: Env, user_id: String) -> ReputationCategory {
    ExternalDataProvider::get_reputation_categories(env.clone())
      .get(user_id)
      .unwrap_or(ReputationCategory::Uncategorized)
  }

  pub fn set_user_reputation_categories(env: Env, reputation_map: Map<String, ReputationCategory>) {
    env
      .storage()
      .instance()
      .set(&DataKey::Reputation, &reputation_map);
  }

  pub fn set_user_reputation_category(env: Env, user_id: String, reputation_category: String) {
    let mut reputation_categories = ExternalDataProvider::get_reputation_categories(env.clone());
    reputation_categories.set(
      user_id,
      reputation_category_from_str(&env, reputation_category),
    );
    env
      .storage()
      .instance()
      .set(&DataKey::Reputation, &reputation_categories);
  }

  pub fn set_users_rep_categories(env: Env, users_reputation_categories: Map<String, String>) {
    let mut all_reputation_categories =
      ExternalDataProvider::get_reputation_categories(env.clone());
    for (user_id, category) in users_reputation_categories {
      all_reputation_categories.set(user_id, reputation_category_from_str(&env, category));
    }
    env
      .storage()
      .instance()
      .set(&DataKey::Reputation, &all_reputation_categories);
  }

  pub fn set_users_rep_categories_vec(
    env: Env,
    users_reputation_categories: Vec<(String, String)>,
  ) {
    let mut all_reputation_categories =
      ExternalDataProvider::get_reputation_categories(env.clone());
    for (user_id, category) in users_reputation_categories {
      all_reputation_categories.set(user_id, reputation_category_from_str(&env, category));
    }
    env
      .storage()
      .instance()
      .set(&DataKey::Reputation, &all_reputation_categories);
  }

  pub fn get_reputation_score(reputation_category: ReputationCategory) -> (u32, u32) {
    match reputation_category {
      ReputationCategory::Uncategorized => (0, 0),
      ReputationCategory::Poor | ReputationCategory::Average => (0, 100),
      ReputationCategory::Good | ReputationCategory::VeryGood => (0, 200),
      ReputationCategory::Excellent => (0, 300),
    }
  }

  // for prior history neuron
  pub fn get_prior_voting_history(env: Env) -> Map<String, Vec<u32>> {
    env
      .storage()
      .instance()
      .get(&DataKey::PriorVotingHistory)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_user_prior_voting_history(env: Env, user_id: String) -> Vec<u32> {
    let voting_history_set: Map<String, Vec<u32>> =
      ExternalDataProvider::get_prior_voting_history(env.clone());
    voting_history_set.get(user_id).unwrap_or(vec![&env])
  }

  pub fn set_user_prior_voting_history(env: Env, user_id: String, new_voting_history: Vec<u32>) {
    let mut voting_history = ExternalDataProvider::get_prior_voting_history(env.clone());
    voting_history.set(user_id, new_voting_history);
    env
      .storage()
      .instance()
      .set(&DataKey::PriorVotingHistory, &voting_history);
  }

  pub fn get_round_bonus_map(env: Env) -> Map<u32, (u32, u32)> {
    let round_bonus_map: Map<u32, (u32, u32)> = env
      .storage()
      .instance()
      .get(&DataKey::RoundBonusMap)
      .unwrap_or(Map::new(&env));
    round_bonus_map
  }

  pub fn set_round_bonus_map(env: Env, round_bonus_map: Map<u32, (u32, u32)>) {
    env
      .storage()
      .instance()
      .set(&DataKey::RoundBonusMap, &round_bonus_map);
  }

  pub fn set_round_bonus_map_vec(env: Env, round_bonus_map: Vec<(u32, u32)>) {
    let mut round_bonus_map_converted = Map::new(&env);
    for (key, val) in round_bonus_map {
      round_bonus_map_converted.set(key, DecimalNumberWrapper::from(val).as_tuple());
    }
    env
      .storage()
      .instance()
      .set(&DataKey::RoundBonusMap, &round_bonus_map_converted);
  }

  // for delegation
  pub fn get_delegation_ranks(env: Env) -> Map<String, u32> {
    env
      .storage()
      .instance()
      .get(&DataKey::DelegationRanks)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_delegation_ranks_for_users(env: Env, users_ids: Vec<String>) -> Map<String, u32> {
    let delegation_ranks = ExternalDataProvider::get_delegation_ranks(env.clone());
    let mut result: Map<String, u32> = Map::new(&env);
    for user_id in users_ids {
      result.set(user_id.clone(), delegation_ranks.get(user_id).unwrap_or(0));
    }
    result
  }

  pub fn set_delegation_rank_for_user(env: Env, user_id: String, new_rank: u32) {
    let mut delegation_ranks = ExternalDataProvider::get_delegation_ranks(env.clone());
    delegation_ranks.set(user_id, new_rank);
    env
      .storage()
      .instance()
      .set(&DataKey::DelegationRanks, &delegation_ranks);
  }

  pub fn set_users_delegation_ranks(env: Env, users_ranks: Map<String, u32>) {
    let mut all_ranks = ExternalDataProvider::get_delegation_ranks(env.clone());
    for (user_id, new_rank) in users_ranks {
      all_ranks.set(user_id, new_rank);
    }
    env
      .storage()
      .instance()
      .set(&DataKey::DelegationRanks, &all_ranks);
  }

  pub fn set_users_delegation_ranks_vec(env: Env, users_ranks: Vec<(String, u32)>) {
    let mut all_ranks = ExternalDataProvider::get_delegation_ranks(env.clone());
    for (user_id, new_rank) in users_ranks {
      all_ranks.set(user_id, new_rank);
    }
    env
      .storage()
      .instance()
      .set(&DataKey::DelegationRanks, &all_ranks);
  }

  // for trust graph neuron
  pub fn get_trust_map(env: Env) -> Map<String, Map<String, ()>> {
    env
      .storage()
      .instance()
      .get(&DataKey::TrustMap)
      .unwrap_or(Map::new(&env))
  }

  pub fn set_trust_map(env: Env, trust_map: Map<String, Map<String, ()>>) {
    env.storage().instance().set(&DataKey::TrustMap, &trust_map);
  }

  pub fn set_trust_map_for_user(
    env: Env,
    user_id: String,
    user_trust_map: Map<String, ()>,
  ) -> Map<String, ()> {
    let mut trust_map = ExternalDataProvider::get_trust_map(env.clone());

    trust_map.set(user_id.clone(), user_trust_map);

    env.storage().instance().set(&DataKey::TrustMap, &trust_map);
    ExternalDataProvider::get_trust_map(env.clone())
      .get(user_id.clone())
      .unwrap_or(Map::new(&env))
  }

  pub fn set_trust_map_for_user_vec(
    env: Env,
    user_id: String,
    user_trust_map: Vec<String>,
  ) -> Map<String, ()> {
    let mut trust_map = ExternalDataProvider::get_trust_map(env.clone());

    let mut new_map = Map::new(&env);
    for item in user_trust_map {
      new_map.set(item, ());
    }

    trust_map.set(user_id.clone(), new_map);

    env.storage().instance().set(&DataKey::TrustMap, &trust_map);
    ExternalDataProvider::get_trust_map(env.clone())
      .get(user_id.clone())
      .unwrap_or(Map::new(&env))
  }

  // for page rank
  pub fn get_page_rank_results(env: Env) -> Map<String, (u32, u32)> {
    env
      .storage()
      .instance()
      .get(&DataKey::PageRankResult)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_page_rank_result_for_user(env: Env, user_id: String) -> (u32, u32) {
    ExternalDataProvider::get_page_rank_results(env.clone())
      .get(user_id)
      .unwrap_or((0, 0))
  }

  pub fn set_page_rank_result(env: Env, new_result: Map<String, (u32, u32)>) {
    env
      .storage()
      .instance()
      .set(&DataKey::PageRankResult, &new_result);
  }

  pub fn calculate_page_rank(env: Env) {
    let trust_map = ExternalDataProvider::get_trust_map(env.clone());

    let page_rank_result = match trust_map.len() {
      0 => Map::new(&env),
      _ => {
        let rank = Rank::from_pages(&env, trust_map);
        rank.calculate(&env)
      }
    };

    ExternalDataProvider::set_page_rank_result(env.clone(), page_rank_result);
  }
}

#[cfg(test)]
mod external_data_provider_test;
