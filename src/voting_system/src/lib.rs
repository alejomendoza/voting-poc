#![no_std]
#![allow(non_upper_case_globals)]

use voting_shared::{
  decimal_number_wrapper::DecimalNumberWrapper,
  types::{DecimalNumber, Vote, VotingSystemError},
};

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Map, Symbol, Vec};

use voting_shared::types::{ProjectUUID, UserUUID};

mod neural_governance_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_neural_governance.wasm"
  );
}

// Address of neural governance contract
const NUERAL_GOVERNANCE: Symbol = symbol_short!("NEURALGOV");
// Map<ProjectUUID, Map<UserUUID, Vote>>
const VOTES: Symbol = symbol_short!("VOTES");
// Vec<ProjectUUID>
const PROJECTS: Symbol = symbol_short!("PROJECTS");

// This contract will be responsible for storing the voting data as well as exposing any needed interface to the users

#[contract]
pub struct VotingSystem;

#[contractimpl]
impl VotingSystem {
  pub fn get_neural_governance(env: Env) -> Result<Address, VotingSystemError> {
    env
      .storage()
      .instance()
      .get(&NUERAL_GOVERNANCE)
      .ok_or(VotingSystemError::NeuralGovernanceNotSet)
  }

  pub fn set_neural_governance(env: Env, neural_governance_address: Address) {
    env
      .storage()
      .instance()
      .set(&NUERAL_GOVERNANCE, &neural_governance_address);
  }

  pub fn vote(
    env: Env,
    voter_id: UserUUID,
    project_id: ProjectUUID,
    vote: Vote,
  ) -> Result<(), VotingSystemError> {
    if !VotingSystem::get_projects(env.clone()).contains(project_id.clone()) {
      return Err(VotingSystemError::ProjectDoesNotExist);
    }

    let mut votes = VotingSystem::get_votes(env.clone());
    let mut project_votes: Map<UserUUID, Vote> =
      votes.get(project_id.clone()).unwrap_or(Map::new(&env));
    if project_votes.contains_key(voter_id.clone()) {
      return Err(VotingSystemError::UserAlreadyVoted);
    }
    project_votes.set(voter_id, vote);
    votes.set(project_id, project_votes);

    env.storage().instance().set(&VOTES, &votes);

    Ok(())
  }

  pub fn get_votes(env: Env) -> Map<ProjectUUID, Map<UserUUID, Vote>> {
    env
      .storage()
      .instance()
      .get(&VOTES)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_projects(env: Env) -> Vec<ProjectUUID> {
    env
      .storage()
      .instance()
      .get(&PROJECTS)
      .unwrap_or(vec![&env])
  }

  pub fn add_project(env: Env, project_id: ProjectUUID) -> Result<(), VotingSystemError> {
    let mut projects = VotingSystem::get_projects(env.clone());
    if projects.contains(project_id.clone()) {
      return Err(VotingSystemError::ProjectAlreadyAdded);
    }
    projects.push_back(project_id);
    env.storage().instance().set(&PROJECTS, &projects);

    Ok(())
  }

  pub fn tally(env: Env) -> Result<Map<ProjectUUID, DecimalNumber>, VotingSystemError> {
    let neural_governance_address = VotingSystem::get_neural_governance(env.clone())?;
    let neural_governance_client =
      neural_governance_contract::Client::new(&env, &neural_governance_address);

    let votes = VotingSystem::get_votes(env.clone());
    let mut result: Map<ProjectUUID, DecimalNumber> = Map::new(&env);
    // ProjectUUID, Map<UserUUID, (Vote, DecimalNumber)>
    for (project_id, votes) in votes {
      let mut project_voting_power_plus: DecimalNumberWrapper = Default::default();
      let mut project_voting_power_minus: DecimalNumberWrapper = Default::default();
      // UserUUID, (Vote, DecimalNumber)
      for (voter_id, vote) in votes {
        let voting_power = match vote {
          Vote::ABSTAIN => (0, 0),
          _ => neural_governance_client.execute_neural_governance(&voter_id, &project_id),
        };
        match vote {
          Vote::YES => {
            project_voting_power_plus = DecimalNumberWrapper::add(
              project_voting_power_plus,
              DecimalNumberWrapper::from(voting_power),
            )
          }
          Vote::NO => {
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
