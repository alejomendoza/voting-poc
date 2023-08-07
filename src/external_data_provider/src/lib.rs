#![no_std]
#![allow(non_upper_case_globals)]

// This contract's going to be responsible for fetching the data from any external resources

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Map, String, Symbol, Vec};
use voting_shared::types::{DecimalNumber, UserUUID};

const REPUTATION: Symbol = symbol_short!("RPUTATION");
const PRIOR_VOTING_HISTORY: Symbol = symbol_short!("PRVTHSTR");
const ROUND_BONUS_MAP: Symbol = symbol_short!("RDBNSMAP");

#[contract]
pub struct ExternalDataProvider;

impl ExternalDataProvider {}

#[contractimpl]
impl ExternalDataProvider {
  pub fn mock_sample_data(env: Env) {
    let mut reputation_map: Map<UserUUID, u32> = Map::new(&env);
    reputation_map.set(String::from_slice(&env, "user001"), 0);
    reputation_map.set(String::from_slice(&env, "user002"), 1);
    reputation_map.set(String::from_slice(&env, "user003"), 2);
    reputation_map.set(String::from_slice(&env, "user004"), 3);
    reputation_map.set(String::from_slice(&env, "user005"), 4);
    env.storage().instance().set(&REPUTATION, &reputation_map);

    let mut voting_history_set: Map<UserUUID, Vec<u32>> = Map::new(&env);
    voting_history_set.set(String::from_slice(&env, "user001"), vec![&env, 2, 3]);
    voting_history_set.set(String::from_slice(&env, "user003"), vec![&env, 2, 3, 4]);
    env
      .storage()
      .instance()
      .set(&PRIOR_VOTING_HISTORY, &voting_history_set);

    let mut round_bonus_map: Map<u32, DecimalNumber> = Map::new(&env);
    round_bonus_map.set(1, (0, 0));
    round_bonus_map.set(2, (0, 100));
    round_bonus_map.set(3, (0, 200));
    round_bonus_map.set(4, (0, 300));
    env
      .storage()
      .instance()
      .set(&ROUND_BONUS_MAP, &round_bonus_map);
  }

  pub fn get_user_reputation_category(env: Env, user_id: UserUUID) -> Option<u32> {
    let map: Map<UserUUID, u32> = Map::new(&env);
    let reputation_map: Map<UserUUID, u32> =
      env.storage().instance().get(&REPUTATION).unwrap_or(map);
    if let Some(user_reputation) = reputation_map.get(user_id) {
      if user_reputation <= 4 {
        return Some(user_reputation);
      }
    }
    None
  }

  pub fn get_user_prior_voting_history(env: Env, user_id: UserUUID) -> Vec<u32> {
    let map: Map<UserUUID, Vec<u32>> = Map::new(&env);
    let voting_history_set: Map<UserUUID, Vec<u32>> = env
      .storage()
      .instance()
      .get(&PRIOR_VOTING_HISTORY)
      .unwrap_or(map);
    voting_history_set.get(user_id).unwrap_or(vec![&env])
  }

  pub fn get_round_bonus_map(env: Env) -> Map<u32, DecimalNumber> {
    let map: Map<u32, DecimalNumber> = Map::new(&env);
    let round_bonus_map: Map<u32, DecimalNumber> = env
      .storage()
      .instance()
      .get(&ROUND_BONUS_MAP)
      .unwrap_or(map);
    round_bonus_map
  }
}

#[cfg(test)]
mod test;
