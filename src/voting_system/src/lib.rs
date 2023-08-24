#![no_std]
#![allow(non_upper_case_globals)]

mod decimal_number_wrapper;
mod layer;
mod neural_governance;
mod neurons;

use crate::decimal_number_wrapper::DecimalNumberWrapper;

use layer::{LayerAggregator, NeuronType};
use neural_governance::NeuralGovernance;
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, Map, String, Vec};
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
    let ng = NeuralGovernance { layers: Vec::new(&env), current_layer_id: 0 };
    env.storage().instance().set(&DataKey::NeuralGovernance, &ng);
  }

  pub fn get_neural_governance(env: Env) -> Result<NeuralGovernance, VotingSystemError> {
    env.storage().instance().get(&DataKey::NeuralGovernance).ok_or(VotingSystemError::NeuralGovernanceNotSet)
  }

  pub fn set_neural_governance(env: Env, neural_governance: NeuralGovernance) {
    env.storage().instance().set(&DataKey::NeuralGovernance, &neural_governance);
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

  // result: map<project_id, project_voting_power>
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

  // implement all operations like: add_layer, add_neuron, set_layer_aggregator, set_neuron_weight, etc.
  // this will do operations on the neural governance contract
  pub fn add_layer(env: Env) -> Result<u32, VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    let new_layer_id = neural_governance.add_layer(env.clone());
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(new_layer_id)
  }

  pub fn set_layer_aggregator(env: Env, layer_id: u32, aggregator: LayerAggregator) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    neural_governance.set_layer_aggregator(layer_id, aggregator)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn add_neuron(env: Env, layer_id: u32, neuron: NeuronType) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    neural_governance.add_neuron(layer_id, neuron)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn set_neuron_weight(env: Env, layer_id: u32, neuron: NeuronType, weight: (u32, u32)) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    neural_governance.set_neuron_weight(layer_id, neuron, weight)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }
}

#[cfg(test)]
mod voting_system_test;
