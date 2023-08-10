#![no_std]
#![allow(non_upper_case_globals)]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};
use voting_shared::{
  decimal_number_persist::DecimalNumberWrapper,
  types::{DecimalNumber, LayerAggregator, ProjectUUID, UserUUID, VotingSystemError},
};

mod simple_neuron_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_simple_neuron.wasm"
  );
}

const AGGREGATOR: Symbol = symbol_short!("AGGRGTR");
const NEURONS: Symbol = symbol_short!("NEURONS");

#[contract]
pub struct Layer;

#[contractimpl]
impl Layer {
  pub fn set_layer_aggregator(env: Env, aggregator: LayerAggregator) {
    env.storage().instance().set(&AGGREGATOR, &aggregator);
  }

  pub fn get_layer_aggregator(env: Env) -> LayerAggregator {
    env
      .storage()
      .instance()
      .get(&AGGREGATOR)
      .unwrap_or(LayerAggregator::UNKNOWN)
  }

  pub fn add_neuron(env: Env, neuron_address: Address) {
    let mut neurons: Vec<Address> = Layer::get_neurons(env.clone());

    neurons.push_back(neuron_address);

    env.storage().instance().set(&NEURONS, &neurons);
  }

  pub fn get_neurons(env: Env) -> Vec<Address> {
    env.storage().instance().get(&NEURONS).unwrap_or(vec![&env])
  }

  pub fn execute_layer(
    env: Env,
    voter_id: UserUUID,
    project_id: ProjectUUID,
    previous_layer_vote: Option<DecimalNumber>,
  ) -> Result<Vec<DecimalNumber>, VotingSystemError> {
    let aggregator: LayerAggregator = Layer::get_layer_aggregator(env.clone());
    if aggregator == LayerAggregator::UNKNOWN {
      return Err(VotingSystemError::LayerAggregatorNotSet);
    }

    let mut neuron_votes: Vec<DecimalNumber> = Vec::new(&env);
    let neurons: Vec<Address> = Layer::get_neurons(env.clone());
    if neurons.is_empty() {
      return Err(VotingSystemError::NoNeuronsExist);
    }
    for neuron in neurons.iter() {
      let neuron_client = simple_neuron_contract::Client::new(&env, &neuron);

      let raw_neuron_vote =
        neuron_client.oracle_function(&voter_id, &project_id, &previous_layer_vote);
      let neuron_vote = neuron_client.weight_function(&raw_neuron_vote);
      neuron_votes.push_back(neuron_vote);
    }
    Ok(neuron_votes)
  }

  pub fn run_layer_aggregator(
    env: Env,
    neuron_votes: Vec<DecimalNumber>,
  ) -> Result<DecimalNumber, VotingSystemError> {
    let aggregator: LayerAggregator = Layer::get_layer_aggregator(env.clone());
    match aggregator {
      LayerAggregator::UNKNOWN => {
        return Err(VotingSystemError::CannotRunUnknownLayerAggregator);
      }
      LayerAggregator::SUM => neuron_votes
        .iter()
        .reduce(|acc, item| {
          let acc = DecimalNumberWrapper::from(acc);
          let item = DecimalNumberWrapper::from(item);
          DecimalNumberWrapper::add(acc, item).as_tuple()
        })
        .ok_or(VotingSystemError::ReducingvotesForSumAggregatorFailed),
      LayerAggregator::PRODUCT => neuron_votes
        .iter()
        .reduce(|acc, item| {
          let acc = DecimalNumberWrapper::from(acc);
          let item = DecimalNumberWrapper::from(item);
          DecimalNumberWrapper::mul(acc, item).as_tuple()
        })
        .ok_or(VotingSystemError::ReducingvotesForProductAggregatorFailed),
    }
  }
}

#[cfg(test)]
mod layer_test;
