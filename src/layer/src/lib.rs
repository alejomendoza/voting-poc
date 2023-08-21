#![no_std]
#![allow(non_upper_case_globals)]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec, String};
use voting_shared::{
  decimal_number_wrapper::DecimalNumberWrapper,
  types::{LayerAggregator, VotingSystemError},
};

mod template_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_template_neuron.wasm"
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
    voter_id: String,
    project_id: String,
    previous_layer_vote: Option<(u32, u32)>,
  ) -> Result<Vec<(u32, u32)>, VotingSystemError> {
    let aggregator: LayerAggregator = Layer::get_layer_aggregator(env.clone());
    if aggregator == LayerAggregator::UNKNOWN {
      return Err(VotingSystemError::LayerAggregatorNotSet);
    }

    let mut neuron_votes: Vec<(u32, u32)> = Vec::new(&env);
    let neurons: Vec<Address> = Layer::get_neurons(env.clone());
    if neurons.is_empty() {
      return Err(VotingSystemError::NoNeuronsExist);
    }
    for neuron in neurons.iter() {
      // even though wemay use different types of neurons here, rust type does not matter
      // in other words, we can inject here any type of neuron and execute it as a template
      // neuron and it seems to work just fine (the functions invoked here are mutual for all neurons)
      let neuron_client = template_neuron_contract::Client::new(&env, &neuron);

      let raw_neuron_vote =
        neuron_client.oracle_function(&voter_id, &project_id, &previous_layer_vote);
      let neuron_vote = neuron_client.weight_function(&raw_neuron_vote);
      neuron_votes.push_back(neuron_vote);
    }
    Ok(neuron_votes)
  }

  pub fn run_layer_aggregator(
    env: Env,
    neuron_votes: Vec<(u32, u32)>,
  ) -> Result<(u32, u32), VotingSystemError> {
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
