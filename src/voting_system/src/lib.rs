#![no_std]
#![allow(non_upper_case_globals)]

mod decimal_number_wrapper;
mod layer;
mod neural_governance;
mod neurons;

use crate::decimal_number_wrapper::DecimalNumberWrapper;

use layer::Layer;
use neural_governance::NeuralGovernance;
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, Map, String, Vec};
use voting_shared::types::{Vote, VotingSystemError};

mod neural_governance_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_neural_governance.wasm"
  );
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  Votes,
  Projects,
  NeuralGovernance,
}

#[contract]
pub struct VotingSystem;

#[contractimpl]
impl VotingSystem {
  pub fn initialize(env: Env) {
    let ng = NeuralGovernance { layers: Vec::new(&env) };
    env.storage().instance().set(&DataKey::NeuralGovernance, &ng);
  }

  pub fn get_neural_governance(env: Env) -> Result<NeuralGovernance, VotingSystemError> {
    env.storage().instance().get(&DataKey::NeuralGovernance).ok_or(VotingSystemError::NeuralGovernanceNotSet)
  }

  pub fn vote(
    env: Env,
    voter_id: String,
    project_id: String,
    vote: Vote,
  ) -> Result<(), VotingSystemError> {
    if !VotingSystem::get_projects(env.clone()).contains(project_id.clone()) {
      return Err(VotingSystemError::ProjectDoesNotExist);
    }

    let mut votes = VotingSystem::get_votes(env.clone());
    let mut project_votes: Map<String, Vote> =
      votes.get(project_id.clone()).unwrap_or(Map::new(&env));
    if project_votes.contains_key(voter_id.clone()) {
      return Err(VotingSystemError::UserAlreadyVoted);
    }
    project_votes.set(voter_id, vote);
    votes.set(project_id, project_votes);

    env.storage().instance().set(&DataKey::Votes, &votes);

    Ok(())
  }

  pub fn get_votes(env: Env) -> Map<String, Map<String, Vote>> {
    env
      .storage()
      .instance()
      .get(&DataKey::Votes)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_projects(env: Env) -> Vec<String> {
    env
      .storage()
      .instance()
      .get(&DataKey::Projects)
      .unwrap_or(vec![&env])
  }

  pub fn add_project(env: Env, project_id: String) -> Result<(), VotingSystemError> {
    let mut projects = VotingSystem::get_projects(env.clone());
    if projects.contains(project_id.clone()) {
      return Err(VotingSystemError::ProjectAlreadyAdded);
    }
    projects.push_back(project_id);
    env.storage().instance().set(&DataKey::Projects, &projects);

    Ok(())
  }

  pub fn tally(env: Env) -> Result<Map<String, (u32, u32)>, VotingSystemError> {
    let votes = VotingSystem::get_votes(env.clone());
    let mut result: Map<String, (u32, u32)> = Map::new(&env);
    // String, Map<String, (Vote, (u32, u32))>
    for (project_id, votes) in votes {
      let mut project_voting_power_plus: DecimalNumberWrapper = Default::default();
      let mut project_voting_power_minus: DecimalNumberWrapper = Default::default();
      // String, (Vote, (u32, u32))
      for (voter_id, vote) in votes {
        let voting_power = match vote {
          Vote::Abstain => (0, 0),
          _ => VotingSystem::get_neural_governance(env.clone())?.execute_neural_governance(env.clone(), voter_id, project_id.clone())?,
        };
        match vote {
          Vote::Yes => {
            project_voting_power_plus = DecimalNumberWrapper::add(
              project_voting_power_plus,
              DecimalNumberWrapper::from(voting_power),
            )
          }
          Vote::No => {
            project_voting_power_minus = DecimalNumberWrapper::add(
              project_voting_power_minus,
              DecimalNumberWrapper::from(voting_power),
            )
          }
          _ => (),
        };
      }
      result.set(
        project_id,
        DecimalNumberWrapper::sub(project_voting_power_plus, project_voting_power_minus).as_tuple(),
      )
    }
    Ok(result)
  }
}

#[cfg(test)]
mod voting_system_test;
