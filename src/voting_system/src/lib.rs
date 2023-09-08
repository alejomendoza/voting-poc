#![no_std]
#![allow(non_upper_case_globals)]

mod decimal_number_wrapper;
mod layer;
mod neural_governance;
mod neurons;
mod types;

use crate::decimal_number_wrapper::DecimalNumberWrapper;

use crate::types::{Vote, VotingSystemError, QUORUM_SIZE};
use layer::{LayerAggregator, NeuronType};
use neural_governance::NeuralGovernance;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Vec};
use types::{ABSTAIN_VOTING_POWER, QUORUM_PARTICIPATION_TRESHOLD};

mod external_data_provider_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  // storage type: instance
  // Map<project_id, Map<user_id, vote>>
  Votes,
  // storage type: instance
  NeuralGovernance,
  // storage type: temporary
  ExternalDataProvider,
}

#[contract]
pub struct VotingSystem;

#[contractimpl]
impl VotingSystem {
  pub fn initialize(env: Env) {
    let ng = NeuralGovernance {
      layers: Vec::new(&env),
      current_layer_id: 0,
    };
    env
      .storage()
      .instance()
      .set(&DataKey::NeuralGovernance, &ng);
  }

  pub fn get_neural_governance(env: Env) -> Result<NeuralGovernance, VotingSystemError> {
    env
      .storage()
      .instance()
      .get(&DataKey::NeuralGovernance)
      .ok_or(VotingSystemError::NeuralGovernanceNotSet)
  }

  pub fn set_neural_governance(env: Env, neural_governance: NeuralGovernance) {
    env
      .storage()
      .instance()
      .set(&DataKey::NeuralGovernance, &neural_governance);
  }

  pub fn calculate_quorum_consensus(
    env: Env,
    voter_id: String,
    project_votes: Map<String, Vote>,
  ) -> Result<Vote, VotingSystemError> {
    let external_data_provider_client = external_data_provider_contract::Client::new(
      &env,
      &VotingSystem::get_external_data_provider(env.clone())?,
    );
    let delegatees = external_data_provider_client
      .get_delegatees()
      .get(voter_id.clone())
      .ok_or(VotingSystemError::DelegateesNotFound)?;
    // delegatees 5-10 have to choose best 5 based on ranks
    let delegation_ranks: Map<String, u32> =
      external_data_provider_client.get_delegation_ranks_for_users(&delegatees.clone());

    let mut sorted_delegatees: Map<String, u32> = Map::new(&env);
    for delegatee_id in delegatees {
      let delegatee_vote = project_votes
        .get(delegatee_id.clone())
        .ok_or(VotingSystemError::VoteNotFoundForDelegatee)?;
      // discard users who delegated
      if delegatee_vote == Vote::Delegate {
        continue;
      }

      let delegatee_rank = delegation_ranks.get(delegatee_id.clone()).unwrap_or(0);

      if sorted_delegatees.clone().len() < QUORUM_SIZE {
        sorted_delegatees.set(delegatee_id.clone(), delegatee_rank);
        continue;
      }
      // find min and if the current is bigger than min then remove them(with min), then insert a new one
      let mut min_rank: Option<(String, u32)> = None;
      for item in sorted_delegatees.clone() {
        let min_rank_clone = min_rank.clone();
        if min_rank_clone.is_none() || item.1 < min_rank_clone.unwrap().1 {
          min_rank = Some(item);
        }
      }
      let min_rank = min_rank.unwrap();
      if delegatee_rank > min_rank.1 {
        sorted_delegatees.remove(min_rank.0);
        sorted_delegatees.set(delegatee_id.clone(), delegatee_rank);
      }
    }

    if sorted_delegatees.clone().len() < QUORUM_SIZE {
      return Ok(Vote::Abstain);
    }

    let mut delegatees_votes: Map<Vote, u32> = Map::new(&env);
    for delegatee in sorted_delegatees {
      let delegatee_vote = project_votes
        .get(delegatee.0.clone())
        .ok_or(VotingSystemError::VoteNotFoundForDelegatee)?;
      if delegatee_vote == Vote::Delegate {
        return Err(VotingSystemError::UnexpectedValue);
      }
      let delegatee_vote_count = delegatees_votes.get(delegatee_vote.clone()).unwrap_or(0);
      delegatees_votes.set(delegatee_vote, delegatee_vote_count + 1);
    }

    let yes_votes = delegatees_votes.get(Vote::Yes).unwrap_or(0);
    let no_votes = delegatees_votes.get(Vote::No).unwrap_or(0);
    let abstain_votes = delegatees_votes.get(Vote::Abstain).unwrap_or(0);
    if abstain_votes >= QUORUM_SIZE - QUORUM_PARTICIPATION_TRESHOLD || yes_votes == no_votes {
      return Ok(Vote::Abstain);
    }
    if yes_votes > no_votes {
      return Ok(Vote::Yes);
    }
    Ok(Vote::No)
  }

  pub fn vote(
    env: Env,
    voter_id: String,
    project_id: String,
    vote: Vote,
  ) -> Result<(), VotingSystemError> {
    let mut votes = VotingSystem::get_votes(env.clone());
    let mut project_votes = votes
      .get(project_id.clone())
      .ok_or(VotingSystemError::ProjectDoesNotExist)?;

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

  pub fn add_project(env: Env, project_id: String) -> Result<(), VotingSystemError> {
    let mut votes = VotingSystem::get_votes(env.clone());
    if votes.get(project_id.clone()).is_some() {
      return Err(VotingSystemError::ProjectAlreadyAdded);
    }
    votes.set(project_id, Map::new(&env));
    env.storage().instance().set(&DataKey::Votes, &votes);

    Ok(())
  }

  // result: map<project_id, project_voting_power>
  pub fn tally(env: Env) -> Result<Map<String, (u32, u32)>, VotingSystemError> {
    let all_votes = VotingSystem::get_votes(env.clone());
    let mut result: Map<String, (u32, u32)> = Map::new(&env);
    // String, Map<String, (Vote, (u32, u32))>
    for (project_id, project_votes) in all_votes {
      let mut project_voting_power_plus: DecimalNumberWrapper = Default::default();
      let mut project_voting_power_minus: DecimalNumberWrapper = Default::default();
      // String, (Vote, (u32, u32))
      for (voter_id, mut vote) in project_votes.clone() {
        if vote == Vote::Delegate {
          vote = VotingSystem::calculate_quorum_consensus(
            env.clone(),
            voter_id.clone(),
            project_votes.clone(),
          )?;
        }
        let voting_power = match vote {
          Vote::Abstain => ABSTAIN_VOTING_POWER,
          _ => VotingSystem::get_neural_governance(env.clone())?.execute_neural_governance(
            env.clone(),
            voter_id.clone(),
            project_id.clone(),
          )?,
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

  pub fn remove_layer(env: Env, layer_id: u32) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    neural_governance.remove_layer(layer_id)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn set_layer_aggregator(
    env: Env,
    layer_id: u32,
    aggregator: LayerAggregator,
  ) -> Result<(), VotingSystemError> {
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

  pub fn remove_neuron(
    env: Env,
    layer_id: u32,
    neuron: NeuronType,
  ) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone()).unwrap();
    neural_governance.remove_neuron(layer_id, neuron)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn set_neuron_weight(
    env: Env,
    layer_id: u32,
    neuron: NeuronType,
    weight: (u32, u32),
  ) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    neural_governance.set_neuron_weight(layer_id, neuron, weight)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn set_external_data_provider(env: Env, external_data_provider_address: Address) {
    env.storage().temporary().set(
      &DataKey::ExternalDataProvider,
      &external_data_provider_address,
    );
  }

  pub fn get_external_data_provider(env: Env) -> Result<Address, VotingSystemError> {
    env
      .storage()
      .temporary()
      .get(&DataKey::ExternalDataProvider)
      .ok_or(VotingSystemError::ExternalDataProviderNotSet)?
  }
}

#[cfg(test)]
mod voting_system_test;
