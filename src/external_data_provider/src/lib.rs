#![no_std]
#![allow(non_upper_case_globals)]

// This contract's going to be responsible for fetching the data from any external resources

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Map, String, Symbol, Vec};
use voting_shared::types::ReputationCategory;

// Map<String, ReputationCategory> - users to their categories
const REPUTATION: Symbol = symbol_short!("RPUTATION");
// Map<String, Vec<u32>> - users to the vector of rounds they participated in
const PRIOR_VOTING_HISTORY: Symbol = symbol_short!("PRVTHSTR");
// Map<u32, (u32, u32)> - (connected to PRIOR_VOTING_HISTORY) rounds to their bonus (for participation)
const ROUND_BONUS_MAP: Symbol = symbol_short!("RDBNSMAP");

#[contract]
pub struct ExternalDataProvider;

impl ExternalDataProvider {}

#[contractimpl]
impl ExternalDataProvider {
  pub fn mock_sample_data(env: Env) {
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
    env.storage().instance().set(&REPUTATION, &reputation_map);

    let mut voting_history_set: Map<String, Vec<u32>> = Map::new(&env);
    voting_history_set.set(String::from_slice(&env, "user001"), vec![&env, 2, 3]);
    voting_history_set.set(String::from_slice(&env, "user003"), vec![&env, 2, 3, 4]);
    env
      .storage()
      .instance()
      .set(&PRIOR_VOTING_HISTORY, &voting_history_set);

    let mut round_bonus_map: Map<u32, (u32, u32)> = Map::new(&env);
    round_bonus_map.set(1, (0, 0));
    round_bonus_map.set(2, (0, 100));
    round_bonus_map.set(3, (0, 200));
    round_bonus_map.set(4, (0, 300));
    env
      .storage()
      .instance()
      .set(&ROUND_BONUS_MAP, &round_bonus_map);
  }

  // for assigned reputation neuron
  pub fn get_user_reputation_category(env: Env, user_id: String) -> ReputationCategory {
    let map: Map<String, ReputationCategory> = Map::new(&env);
    let reputation_map: Map<String, ReputationCategory> =
      env.storage().instance().get(&REPUTATION).unwrap_or(map);
    reputation_map
      .get(user_id)
      .unwrap_or(ReputationCategory::Uncategorized)
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
  pub fn get_user_prior_voting_history(env: Env, user_id: String) -> Vec<u32> {
    let map: Map<String, Vec<u32>> = Map::new(&env);
    let voting_history_set: Map<String, Vec<u32>> = env
      .storage()
      .instance()
      .get(&PRIOR_VOTING_HISTORY)
      .unwrap_or(map);
    voting_history_set.get(user_id).unwrap_or(vec![&env])
  }

  pub fn get_round_bonus_map(env: Env) -> Map<u32, (u32, u32)> {
    let map: Map<u32, (u32, u32)> = Map::new(&env);
    let round_bonus_map: Map<u32, (u32, u32)> = env
      .storage()
      .instance()
      .get(&ROUND_BONUS_MAP)
      .unwrap_or(map);
    round_bonus_map
  }
}

#[cfg(test)]
mod external_data_provider_test;
