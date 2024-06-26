#![no_std]
#![allow(non_upper_case_globals)]

mod layer;
mod neural_governance;
mod neurons;
mod types;

use crate::types::{Vote, VotingSystemError, QUORUM_SIZE};
use neural_governance::NeuralGovernance;
use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Vec};
use types::{
  layer_aggregator_from_str, neuron_type_from_str, normalized_vote_from_str, vote_from_str,
  LayerAggregator, NormalizedVote, ABSTAIN_VOTING_POWER, DEFAULT_WEIGHT, MAX_DELEGATEES,
  MIN_DELEGATEES, QUORUM_PARTICIPATION_TRESHOLD,
};

mod external_data_provider_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  // storage type: instance
  // Map<submission_id, Map<user_id, vote>>
  Votes,
  // storage type: instance
  NeuralGovernance,
  // storage type: instance
  // Map<user_id, Vec<user_id>> - users to the vector of users they delegated their votes to
  Delegatees,
  // storage type: instance
  ExternalDataProvider,
  // storage type: instance
  // Map<user_id, (u32, u32)>
  VotingPowers,
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

  fn set_neural_governance(env: Env, neural_governance: NeuralGovernance) {
    env
      .storage()
      .instance()
      .set(&DataKey::NeuralGovernance, &neural_governance);
  }

  pub fn calculate_quorum_consensus(
    env: Env,
    voter_id: String,
    submission_id: String,
  ) -> Result<Vote, VotingSystemError> {
    let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_address);

    let delegatees = VotingSystem::get_delegatees(env.clone())
      .get(voter_id.clone())
      .ok_or(VotingSystemError::DelegateesNotFound)?;
    // delegatees 5-10 have to choose best 5 based on ranks
    let delegation_ranks: Map<String, u32> =
      external_data_provider_client.get_delegation_ranks_for_users(&delegatees.clone());

    let all_votes = VotingSystem::get_votes(env.clone());
    let submission_votes = all_votes
      .get(submission_id.clone())
      .unwrap_or(Map::new(&env));

    let mut sorted_delegatees: Map<String, u32> = Map::new(&env);
    for delegatee_id in delegatees {
      let delegatee_vote = submission_votes
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
      let delegatee_vote = submission_votes
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

  pub fn multiple_vote_operations(
    env: Env,
    voter_id: String,
    votes: Map<String, String>,
  ) -> Result<Map<String, Vote>, VotingSystemError> {
    let mut all_votes = VotingSystem::get_votes(env.clone());
    for (submission_id, vote) in votes {
      let vote: Vote = vote_from_str(&env, vote);

      let mut submission_votes = all_votes
        .get(submission_id.clone())
        .unwrap_or(Map::new(&env));
      if vote == Vote::Remove {
        submission_votes.remove(voter_id.clone());
      } else {
        submission_votes.set(voter_id.clone(), vote);
      }

      if submission_votes.is_empty() {
        all_votes.remove(submission_id.clone());
      } else {
        all_votes.set(submission_id, submission_votes);
      }
    }

    env.storage().instance().set(&DataKey::Votes, &all_votes);
    Ok(VotingSystem::get_votes_for_user(env, voter_id))
  }

  // votes: Map<submission_id, vote>
  pub fn multiple_vote_operations_vec(
    env: Env,
    voter_id: String,
    // TODO this should be a map but soroban's maps are buggy so we use vector of tuples
    // map would not work when used as an argument for specific keys and would just throw errors
    votes_vec: Vec<(String, String)>,
  ) -> Result<Map<String, Vote>, VotingSystemError> {
    let mut votes: Map<String, String> = Map::new(&env);
    for (submission_id, vote) in votes_vec {
      votes.set(submission_id, vote);
    }
    VotingSystem::multiple_vote_operations(env, voter_id, votes)
  }

  pub fn vote(
    env: Env,
    voter_id: String,
    submission_id: String,
    vote: String,
  ) -> Result<Map<String, Vote>, VotingSystemError> {
    let vote: Vote = vote_from_str(&env, vote);
    if vote == Vote::Delegate
      && VotingSystem::get_delegatees(env.clone())
        .get(voter_id.clone())
        .is_none()
    {
      return Err(VotingSystemError::DelegateesNotFound);
    }

    let mut votes = VotingSystem::get_votes(env.clone());
    let mut submission_votes: Map<String, Vote> =
      votes.get(submission_id.clone()).unwrap_or(Map::new(&env));

    if vote == Vote::Remove {
      submission_votes.remove(voter_id.clone());
    } else {
      submission_votes.set(voter_id.clone(), vote);
    }

    if submission_votes.is_empty() {
      votes.remove(submission_id.clone());
    } else {
      votes.set(submission_id, submission_votes);
    }

    env.storage().instance().set(&DataKey::Votes, &votes);

    Ok(VotingSystem::get_votes_for_user(env, voter_id))
  }

  pub fn get_delegatees(env: Env) -> Map<String, Vec<String>> {
    env
      .storage()
      .instance()
      .get(&DataKey::Delegatees)
      .unwrap_or(Map::new(&env))
  }

  pub fn set_delegatees(
    env: Env,
    voter_id: String,
    delegatees_for_user: Vec<String>,
  ) -> Result<Vec<String>, VotingSystemError> {
    if delegatees_for_user.len() > MAX_DELEGATEES {
      return Err(VotingSystemError::TooManyDelegatees);
    }
    if delegatees_for_user.len() < MIN_DELEGATEES {
      return Err(VotingSystemError::NotEnoughDelegatees);
    }
    let mut all_delegatees = VotingSystem::get_delegatees(env.clone());
    all_delegatees.set(voter_id.clone(), delegatees_for_user);
    env
      .storage()
      .instance()
      .set(&DataKey::Delegatees, &all_delegatees);

    Ok(
      VotingSystem::get_delegatees(env.clone())
        .get(voter_id.clone())
        .unwrap_or(Vec::new(&env)),
    )
  }

  pub fn delegate(
    env: Env,
    voter_id: String,
    submission_id: String,
    delegatees_for_user: Vec<String>,
  ) -> Result<Map<String, Vote>, VotingSystemError> {
    VotingSystem::set_delegatees(env.clone(), voter_id.clone(), delegatees_for_user)?;
    VotingSystem::vote(
      env.clone(),
      voter_id,
      submission_id,
      String::from_slice(&env, "Delegate"),
    )
  }

  pub fn get_votes(env: Env) -> Map<String, Map<String, Vote>> {
    env
      .storage()
      .instance()
      .get(&DataKey::Votes)
      .unwrap_or(Map::new(&env))
  }

  pub fn get_votes_length(env: Env) -> u32 {
    let votes: Map<String, Map<String, Vote>> = env
      .storage()
      .instance()
      .get(&DataKey::Votes)
      .unwrap_or(Map::new(&env));
    votes.len()
  }

  pub fn get_votes_for_user(env: Env, voter_id: String) -> Map<String, Vote> {
    let all_votes: Map<String, Map<String, Vote>> = env
      .storage()
      .instance()
      .get(&DataKey::Votes)
      .unwrap_or(Map::new(&env));
    // submission id => vote
    let mut result: Map<String, Vote> = Map::new(&env);
    for (submission_id, submission_votes) in all_votes {
      if let Some(vote) = submission_votes.get(voter_id.clone()) {
        result.set(submission_id, vote);
      }
    }
    result
  }

  pub fn remove_vote(
    env: Env,
    voter_id: String,
    submission_id: String,
  ) -> Result<Map<String, Vote>, VotingSystemError> {
    VotingSystem::vote(
      env.clone(),
      voter_id,
      submission_id,
      String::from_slice(&env, "Remove"),
    )
  }

  pub fn add_submission(env: Env, submission_id: String) -> Result<(), VotingSystemError> {
    let mut votes = VotingSystem::get_votes(env.clone());
    if votes.get(submission_id.clone()).is_some() {
      return Err(VotingSystemError::SubmissionAlreadyAdded);
    }
    votes.set(submission_id, Map::new(&env));
    env.storage().instance().set(&DataKey::Votes, &votes);

    Ok(())
  }

  pub fn get_submissions(env: Env) -> Vec<String> {
    VotingSystem::get_votes(env.clone()).keys()
  }

  pub fn get_voters(env: Env) -> Vec<String> {
    let votes = VotingSystem::get_votes(env.clone());
    let mut voters: Map<String, ()> = Map::new(&env);
    for (_, submission_votes) in votes {
      for voter_id in submission_votes.keys() {
        voters.set(voter_id, ());
      }
    }
    voters.keys()
  }

  // result: map<submission_id, submission_voting_power>
  pub fn tally(env: Env) -> Result<Map<String, (u32, u32)>, VotingSystemError> {
    let all_votes = VotingSystem::get_votes(env.clone());
    let mut result: Map<String, (u32, u32)> = Map::new(&env);
    // String, Map<String, (Vote, (u32, u32))>
    for (submission_id, submission_votes) in all_votes {
      let mut submission_voting_power_plus: DecimalNumberWrapper = Default::default();
      let mut submission_voting_power_minus: DecimalNumberWrapper = Default::default();
      // String, (Vote, (u32, u32))
      for (voter_id, mut vote) in submission_votes.clone() {
        if vote == Vote::Delegate {
          vote = VotingSystem::calculate_quorum_consensus(
            env.clone(),
            voter_id.clone(),
            submission_id.clone(),
          )?;
        }
        let voting_power = match vote {
          Vote::Abstain => ABSTAIN_VOTING_POWER,
          _ => VotingSystem::get_neural_governance(env.clone())?.execute_neural_governance(
            env.clone(),
            voter_id.clone(),
            submission_id.clone(),
          )?,
        };
        match vote {
          Vote::Yes => {
            submission_voting_power_plus = DecimalNumberWrapper::add(
              submission_voting_power_plus,
              DecimalNumberWrapper::from(voting_power),
            )
          }
          Vote::No => {
            submission_voting_power_minus = DecimalNumberWrapper::add(
              submission_voting_power_minus,
              DecimalNumberWrapper::from(voting_power),
            )
          }
          _ => (),
        };
      }
      result.set(
        submission_id,
        DecimalNumberWrapper::sub(submission_voting_power_plus, submission_voting_power_minus)
          .as_tuple(),
      )
    }
    Ok(result)
  }

  /**
   * This is a breakdown of the tally function, instead of tally, you can call:
   * 1. normalize_votes
   * 2. voting_power_for_voter - for every user/submission (this depends on the fact whether any neurons consider submission id in calculations) - you should iterate over the result of normalize_votes
   * 3. submissions_voting_powers - with the results of normalize_votes and voting_power_for_voter
   *
   * tally reaches the CPU Soroban limit (https://soroban.stellar.org/docs/fundamentals-and-concepts/fees-and-metering#resource-limits) pretty quickly, with this breakdown you can run it in batches
   */

  // this will convert all the votes to either Yes or No
  // returns Map<submission_id, Map<voter_id, normalized_vote>>
  pub fn normalize_votes(env: Env) -> Result<Map<String, Map<String, String>>, VotingSystemError> {
    // Map<submission_id, Map<voter_id, Vote>>
    let all_votes = VotingSystem::get_votes(env.clone());
    let mut normalized_votes: Map<String, Map<String, String>> = Map::new(&env);
    for (submission_id, submission_votes) in all_votes {
      // Map<voter_id, normalized_vote>
      let mut normalized_votes_for_submission: Map<String, String> = Map::new(&env);
      for (voter_id, mut vote) in submission_votes.clone() {
        if vote == Vote::Delegate {
          vote = VotingSystem::calculate_quorum_consensus(
            env.clone(),
            voter_id.clone(),
            submission_id.clone(),
          )?;
        }
        if vote == Vote::Abstain {
          continue;
        }
        if vote == Vote::Yes {
          normalized_votes_for_submission.set(voter_id, String::from_slice(&env, "Yes"));
        } else if vote == Vote::No {
          normalized_votes_for_submission.set(voter_id, String::from_slice(&env, "No"));
        } else {
          return Err(VotingSystemError::UnexpectedValue);
        }
      }
      if !normalized_votes_for_submission.is_empty() {
        normalized_votes.set(submission_id, normalized_votes_for_submission);
      }
    }
    Ok(normalized_votes)
  }

  // normalize votes but just override the collection in place (in the storage) and do not return anything
  // they will be referred to in submissions_voting_powers (maybe normalized can be saved in a separate collection in storage)

  // returns Map<voter_id, normalized_vote>
  pub fn normalize_votes_for_submission(
    env: Env,
    submission_id: String,
  ) -> Result<Map<String, String>, VotingSystemError> {
    let submission_votes = VotingSystem::get_votes(env.clone())
      .get(submission_id.clone())
      .unwrap_or(Map::new(&env));

    let mut result: Map<String, String> = Map::new(&env);

    for (voter_id, mut vote) in submission_votes {
      if vote == Vote::Delegate {
        vote = VotingSystem::calculate_quorum_consensus(
          env.clone(),
          voter_id.clone(),
          submission_id.clone(),
        )?;
      }
      if vote == Vote::Abstain {
        continue;
      }
      if vote == Vote::Yes {
        result.set(voter_id, String::from_slice(&env, "Yes"));
      } else if vote == Vote::No {
        result.set(voter_id, String::from_slice(&env, "No"));
      } else {
        return Err(VotingSystemError::UnexpectedValue);
      }
    }

    Ok(result)
  }

  // this calls a neural governance for every voter and submission
  pub fn voting_power_for_voter(
    env: Env,
    voter_id: String,
    submission_id: String,
  ) -> Result<(u32, u32), VotingSystemError> {
    let voting_power = VotingSystem::get_neural_governance(env.clone())?
      .execute_neural_governance(env.clone(), voter_id.clone(), submission_id.clone())?;

    VotingSystem::set_voting_power_for_user(env, voter_id, voting_power);

    Ok(voting_power)
  }

  pub fn get_voting_powers(env: Env) -> Map<String, (u32, u32)> {
    env
      .storage()
      .instance()
      .get(&DataKey::VotingPowers)
      .unwrap_or(Map::new(&env))
  }

  pub fn set_voting_powers(env: Env, new_voting_powers: Vec<(String, u32)>) {
    let mut voting_powers = VotingSystem::get_voting_powers(env.clone());

    for (voter_id, voting_power) in new_voting_powers {
      voting_powers.set(
        voter_id,
        DecimalNumberWrapper::from(voting_power).as_tuple(),
      );
    }

    env
      .storage()
      .instance()
      .set(&DataKey::VotingPowers, &voting_powers);
  }

  pub fn set_voting_power_for_user(env: Env, voter_id: String, voting_power: (u32, u32)) {
    let mut voting_powers: Map<String, (u32, u32)> = env
      .storage()
      .instance()
      .get(&DataKey::VotingPowers)
      .unwrap_or(Map::new(&env));

    voting_powers.set(voter_id.clone(), voting_power);

    env
      .storage()
      .instance()
      .set(&DataKey::VotingPowers, &voting_powers);
  }

  pub fn submissions_voting_powers(
    env: Env,
    // Map<user_id, voting_power>
    voters_voting_powers: Map<String, u32>,
    // Map<submission_id, Map<user_id, normalized_vote>>
    normalized_votes: Map<String, Map<String, String>>,
  ) -> Result<Map<String, (u32, u32)>, VotingSystemError> {
    let mut result: Map<String, (u32, u32)> = Map::new(&env);

    for (submission_id, votes) in normalized_votes {
      let mut submission_voting_power_plus: DecimalNumberWrapper = Default::default();
      let mut submission_voting_power_minus: DecimalNumberWrapper = Default::default();
      for (voter_id, normalized_vote_str) in votes {
        let normalized_vote = normalized_vote_from_str(&env, normalized_vote_str)?;
        let voter_voting_power: Option<u32> = voters_voting_powers.get(voter_id);
        if voter_voting_power.is_none() {
          return Err(VotingSystemError::UnknownVoter);
        }
        let voter_voting_power: u32 = voter_voting_power.unwrap();
        let voter_voting_power: (u32, u32) =
          DecimalNumberWrapper::from(voter_voting_power).as_tuple();
        match normalized_vote {
          NormalizedVote::Yes => {
            submission_voting_power_plus = DecimalNumberWrapper::add(
              submission_voting_power_plus,
              DecimalNumberWrapper::from(voter_voting_power),
            )
          }
          NormalizedVote::No => {
            submission_voting_power_minus = DecimalNumberWrapper::add(
              submission_voting_power_minus,
              DecimalNumberWrapper::from(voter_voting_power),
            )
          }
        };
      }
      result.set(
        submission_id,
        DecimalNumberWrapper::sub(submission_voting_power_plus, submission_voting_power_minus)
          .as_tuple(),
      )
    }

    Ok(result)
  }

  // takes results of neural governance runs and calculates the final voting power for each submission
  pub fn submissions_voting_powers_vec(
    env: Env,
    // Map<user_id, voting_power>
    voters_voting_powers_vec: Vec<(String, u32)>,
    // Map<submission_id, Map<user_id, normalized_vote>>
    normalized_votes_vec: Vec<(String, String, String)>,
  ) -> Result<Map<String, (u32, u32)>, VotingSystemError> {
    let mut voters_voting_powers: Map<String, u32> = Map::new(&env);
    for (user_id, voting_power) in voters_voting_powers_vec {
      voters_voting_powers.set(user_id, voting_power);
    }

    let mut normalized_votes: Map<String, Map<String, String>> = Map::new(&env);
    for (submission_id, voter_id, vote) in normalized_votes_vec {
      if normalized_votes.get(submission_id.clone()).is_none() {
        normalized_votes.set(submission_id.clone(), Map::new(&env));
      }
      let mut current = normalized_votes.get(submission_id.clone()).unwrap();
      current.set(voter_id.clone(), vote.clone());
      normalized_votes.set(submission_id.clone(), current);
    }
    VotingSystem::submissions_voting_powers(env, voters_voting_powers, normalized_votes)
  }

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
    aggregator: String,
  ) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    let layer_aggregator: LayerAggregator = layer_aggregator_from_str(&env, aggregator);
    neural_governance.set_layer_aggregator(layer_id, layer_aggregator)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn add_neuron(env: Env, layer_id: u32, neuron: String) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    let neuron = neuron_type_from_str(&env, neuron)?;
    neural_governance.add_neuron(layer_id, neuron)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn remove_neuron(env: Env, layer_id: u32, neuron: String) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone()).unwrap();
    let neuron = neuron_type_from_str(&env, neuron)?;
    neural_governance.remove_neuron(layer_id, neuron)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  pub fn set_neuron_weight(
    env: Env,
    layer_id: u32,
    neuron: String,
    weight: u32,
  ) -> Result<(), VotingSystemError> {
    let mut neural_governance = VotingSystem::get_neural_governance(env.clone())?;
    let neuron = neuron_type_from_str(&env, neuron)?;
    let weight = DecimalNumberWrapper::from(weight).as_tuple();
    neural_governance.set_neuron_weight(layer_id, neuron, weight)?;
    VotingSystem::set_neural_governance(env, neural_governance);
    Ok(())
  }

  // todo test this method
  pub fn setup_layer(
    env: Env,
    layer_aggregator: String,
    neurons: Vec<(String, u32)>,
  ) -> Result<(), VotingSystemError> {
    let layer_id = VotingSystem::add_layer(env.clone())?;
    VotingSystem::set_layer_aggregator(env.clone(), layer_id, layer_aggregator)?;

    for (neuron, neuron_weight) in neurons {
      VotingSystem::add_neuron(env.clone(), layer_id, neuron.clone())?;
      if DecimalNumberWrapper::from(neuron_weight).as_tuple() != DEFAULT_WEIGHT
        && neuron_weight != 0
      {
        VotingSystem::set_neuron_weight(env.clone(), layer_id, neuron.clone(), neuron_weight)?;
      }
    }
    Ok(())
  }

  pub fn set_external_data_provider(env: Env, external_data_provider_address: Address) {
    env.storage().instance().set(
      &DataKey::ExternalDataProvider,
      &external_data_provider_address,
    );
  }

  pub fn get_external_data_provider(env: Env) -> Result<Address, VotingSystemError> {
    env
      .storage()
      .instance()
      .get(&DataKey::ExternalDataProvider)
      .ok_or(VotingSystemError::ExternalDataProviderNotSet)?
  }

  pub fn calculate_page_rank(env: Env) -> Result<(), VotingSystemError> {
    let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_address);

    external_data_provider_client.calculate_page_rank();

    Ok(())
  }
}

#[cfg(test)]
mod voting_system_test;
