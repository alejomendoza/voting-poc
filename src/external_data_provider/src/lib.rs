#![no_std]
#![allow(non_upper_case_globals)]

pub mod types;

// This contract's going to be responsible for fetching the data from any external resources

use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, Map, String, Vec};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReputationCategory {
  Excellent = 5,
  VeryGood = 4,
  Good = 3,
  Average = 2,
  Poor = 1,
  Uncategorized = 0,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  // storage type: temporary
  // Map<UserUUID, ReputationCategory> - users to their categories
  Reputation,
  // storage type: temporary
  // Map<UserUUID, Vec<u32>> - users to the vector of rounds they participated in
  PriorVotingHistory,
  // storage type: temporary
  // Map<u32, DecimalNumber> - (connected to PRIOR_VOTING_HISTORY) rounds to their bonus (for participation)
  RoundBonusMap,
  // storage type: temporary
  // Map<UserUUID, u32> - users to their delegation rank
  DelegationRanks,
}

#[contract]
pub struct ExternalDataProvider;

impl ExternalDataProvider {}

#[contractimpl]
impl ExternalDataProvider {
  pub fn mock_sample_data(env: Env) {
    // for assigned reputation neuron
    let mut reputation_map: Map<String, ReputationCategory> = Map::new(&env);
    reputation_map.set(
      String::from_slice(&env, "user001"),
      ReputationCategory::Excellent,
    );
    reputation_map.set(
      String::from_slice(&env, "user002"),
      ReputationCategory::VeryGood,
    );
    reputation_map.set(
      String::from_slice(&env, "user003"),
      ReputationCategory::Good,
    );
    reputation_map.set(
      String::from_slice(&env, "user004"),
      ReputationCategory::Average,
    );
    reputation_map.set(
      String::from_slice(&env, "user005"),
      ReputationCategory::Poor,
    );
    env
      .storage()
      .temporary()
      .set(&DataKey::Reputation, &reputation_map);

    // for prior history neuron
    let mut voting_history_set: Map<String, Vec<u32>> = Map::new(&env);
    voting_history_set.set(String::from_slice(&env, "user001"), vec![&env, 2, 3]);
    voting_history_set.set(String::from_slice(&env, "user003"), vec![&env, 2, 3, 4]);
    env
      .storage()
      .temporary()
      .set(&DataKey::PriorVotingHistory, &voting_history_set);

    let mut round_bonus_map: Map<u32, (u32, u32)> = Map::new(&env);
    round_bonus_map.set(1, (0, 0));
    round_bonus_map.set(2, (0, 100));
    round_bonus_map.set(3, (0, 200));
    round_bonus_map.set(4, (0, 300));
    env
      .storage()
      .temporary()
      .set(&DataKey::RoundBonusMap, &round_bonus_map);

    let mut delegation_ranks: Map<String, u32> = Map::new(&env);
    delegation_ranks.set(String::from_slice(&env, "user001"), 1);
    delegation_ranks.set(String::from_slice(&env, "user002"), 2);
    delegation_ranks.set(String::from_slice(&env, "user003"), 3);
    delegation_ranks.set(String::from_slice(&env, "user004"), 4);
    delegation_ranks.set(String::from_slice(&env, "user005"), 5);
    delegation_ranks.set(String::from_slice(&env, "user006"), 6);
    delegation_ranks.set(String::from_slice(&env, "user007"), 7);
    delegation_ranks.set(String::from_slice(&env, "user008"), 8);
    env
      .storage()
      .temporary()
      .set(&DataKey::DelegationRanks, &delegation_ranks);
  }

  // for assigned reputation neuron
  pub fn get_reputation_categories(env: Env) -> Map<String, ReputationCategory> {
    env
      .storage()
      .temporary()
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
      .temporary()
      .set(&DataKey::Reputation, &reputation_map);
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
      .temporary()
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
      .temporary()
      .set(&DataKey::PriorVotingHistory, &voting_history);
  }

  pub fn get_round_bonus_map(env: Env) -> Map<u32, (u32, u32)> {
    let round_bonus_map: Map<u32, (u32, u32)> = env
      .storage()
      .temporary()
      .get(&DataKey::RoundBonusMap)
      .unwrap_or(Map::new(&env));
    round_bonus_map
  }

  pub fn set_round_bonus_map(env: Env, round_bonus_map: Map<u32, (u32, u32)>) {
    env
      .storage()
      .temporary()
      .set(&DataKey::RoundBonusMap, &round_bonus_map);
  }

  // for delegation
  pub fn get_delegation_ranks(env: Env) -> Map<String, u32> {
    env
      .storage()
      .temporary()
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
      .temporary()
      .set(&DataKey::DelegationRanks, &delegation_ranks);
  }
}

#[cfg(test)]
mod external_data_provider_test;
