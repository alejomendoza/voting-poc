use soroban_sdk::{contracttype, vec, Address, Env, String, Vec};
use voting_shared::types::VotingSystemError;

use crate::decimal_number_wrapper::DecimalNumberWrapper;

mod template_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_template_neuron.wasm"
  );
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayerAggregator {
  Unknown,
  Sum,
  Product,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
  Neurons,
  Aggregator,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Layer {
  pub neurons: Vec<Address>,
  pub aggregator: LayerAggregator,
}

impl Layer {
  pub fn add_neuron(env: Env, neuron_address: Address) {
    let mut neurons: Vec<Address> = Layer::get_neurons(env.clone());
    neurons.push_back(neuron_address);
    env.storage().instance().set(&DataKey::Neurons, &neurons);
  }

  pub fn get_neurons(env: Env) -> Vec<Address> {
    env
      .storage()
      .instance()
      .get(&DataKey::Neurons)
      .unwrap_or(vec![&env])
  }

  pub fn execute_layer(
    &self,
    env: Env,
    voter_id: String,
    project_id: String,
    previous_layer_vote: Option<(u32, u32)>,
  ) -> Result<Vec<(u32, u32)>, VotingSystemError> {
    if self.aggregator == LayerAggregator::Unknown {
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
    &self,
    env: Env,
    neuron_votes: Vec<(u32, u32)>,
  ) -> Result<(u32, u32), VotingSystemError> {
    match self.aggregator {
      LayerAggregator::Unknown => {
        return Err(VotingSystemError::CannotRunUnknownLayerAggregator);
      }
      LayerAggregator::Sum => neuron_votes
        .iter()
        .reduce(|acc, item| {
          let acc = DecimalNumberWrapper::from(acc);
          let item = DecimalNumberWrapper::from(item);
          DecimalNumberWrapper::add(acc, item).as_tuple()
        })
        .ok_or(VotingSystemError::ReducingvotesForSumAggregatorFailed),
      LayerAggregator::Product => neuron_votes
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
