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
  // storage type: temporary
  // Map<UserUUID, Vec<UserUUID>> - users to their delegation rank
  TrustMap,
}

#[contract]
pub struct ExternalDataProvider;

impl ExternalDataProvider {}

#[contractimpl]
impl ExternalDataProvider {
  pub fn mock_sample_data(env: Env) {
    let user001 = String::from_slice(&env, "user001");
    let user002 = String::from_slice(&env, "user002");
    let user003 = String::from_slice(&env, "user003");
    let user004 = String::from_slice(&env, "user004");
    let user005 = String::from_slice(&env, "user005");
    let user006 = String::from_slice(&env, "user006");
    let user007 = String::from_slice(&env, "user007");
    let user008 = String::from_slice(&env, "user008");
    let user009 = String::from_slice(&env, "user009");
    // for assigned reputation neuron
    let mut reputation_map: Map<String, ReputationCategory> = Map::new(&env);
    reputation_map.set(user001.clone(), ReputationCategory::Excellent);
    reputation_map.set(user002.clone(), ReputationCategory::VeryGood);
    reputation_map.set(user003.clone(), ReputationCategory::Good);
    reputation_map.set(user004.clone(), ReputationCategory::Average);
    reputation_map.set(user005.clone(), ReputationCategory::Poor);
    env
      .storage()
      .temporary()
      .set(&DataKey::Reputation, &reputation_map);

    // for prior history neuron
    let mut voting_history_set: Map<String, Vec<u32>> = Map::new(&env);
    voting_history_set.set(user001.clone(), vec![&env, 2, 3]);
    voting_history_set.set(user003.clone(), vec![&env, 2, 3, 4]);
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
    delegation_ranks.set(user001.clone(), 1);
    delegation_ranks.set(user002.clone(), 2);
    delegation_ranks.set(user003.clone(), 3);
    delegation_ranks.set(user004.clone(), 4);
    delegation_ranks.set(user005.clone(), 5);
    delegation_ranks.set(user006.clone(), 6);
    delegation_ranks.set(user007.clone(), 7);
    delegation_ranks.set(user008.clone(), 8);
    env
      .storage()
      .temporary()
      .set(&DataKey::DelegationRanks, &delegation_ranks);

    // for trust graph neuron
    let mut trust_map: Map<String, Vec<String>> = Map::new(&env);
    trust_map.set(
      user001.clone(),
      vec![&env, user002.clone(), user004.clone()],
    );
    trust_map.set(
      user002.clone(),
      vec![&env, user001.clone(),]
    );
    trust_map.set(
      user003.clone(),
      vec![&env, user001.clone(), user002.clone(),]
    );
    trust_map.set(
      user004.clone(),
      vec![&env, user003.clone(),]
    );
    env
      .storage()
      .temporary()
      .set(&DataKey::TrustMap, &trust_map);
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

  // for trust graph neuron
  pub fn get_trust_map(env: Env) -> Map<String, Vec<String>> {
    env
      .storage()
      .temporary()
      .get(&DataKey::TrustMap)
      .unwrap_or(Map::new(&env))
  }
  pub fn set_trust_map(env: Env, trust_map: Map<String, Vec<String>>) {
    env
      .storage()
      .temporary()
      .set(&DataKey::TrustMap, &trust_map);
  }
}

#[cfg(test)]
mod external_data_provider_test;
