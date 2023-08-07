#![no_std]
#![allow(non_upper_case_globals)]

// This contract's going to be responsible for fetching the data from any external resources

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String, Symbol};
use voting_shared::types::UserUUID;

const REPUTATION: Symbol = symbol_short!("RPUTATION");

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
  }

  pub fn get_reputation_category_for_user(env: Env, user_id: UserUUID) -> Option<u32> {
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
}

#[cfg(test)]
mod test;
