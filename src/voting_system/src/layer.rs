use soroban_sdk::{contracttype, vec, Address, Env, Map, String, TryFromVal, Vec};
use voting_shared::types::{DecimalNumber, VotingSystemError};

use crate::{decimal_number_wrapper::DecimalNumberWrapper, neurons::dummy_neuron};

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

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NeuronType {
  Dummy,
  AssignedReputation,
  PriorVotingHistory,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Layer {
  // neuron type, neuron raw weight
  pub neurons: Map<NeuronType, u32>,
  pub aggregator: LayerAggregator,
}

impl Layer {
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
    if self.neurons.is_empty() {
      return Err(VotingSystemError::NoNeuronsExist);
    }
    for (neuron, raw_weight) in self.neurons.iter() {
      // even though wemay use different types of neurons here, rust type does not matter
      // in other words, we can inject here any type of neuron and execute it as a template
      // neuron and it seems to work just fine (the functions invoked here are mutual for all neurons)
      // let neuron_client = template_neuron_contract::Client::new(&env, &neuron);

      // let raw_neuron_vote =
      //   neuron_client.oracle_function(&voter_id, &project_id, &previous_layer_vote);
      // let neuron_vote = neuron_client.weight_function(&raw_neuron_vote);
      let raw_neuron_vote: DecimalNumber = match neuron {
        NeuronType::Dummy => dummy_neuron::oracle_function(
          env.clone(),
          voter_id.clone(),
          project_id.clone(),
          previous_layer_vote,
        )?,
        NeuronType::AssignedReputation => unimplemented!(),
        NeuronType::PriorVotingHistory => unimplemented!(),
      };
      let neuron_vote = self.run_neuron_weight_function(
        env.clone(),
        raw_neuron_vote,
        DecimalNumberWrapper::from(raw_weight).as_tuple(),
      );
      neuron_votes.push_back(neuron_vote);
    }
    Ok(neuron_votes)
  }

  fn run_neuron_weight_function(
    &self,
    env: Env,
    raw_neuron_vote: (u32, u32),
    weight: DecimalNumber,
  ) -> (u32, u32) {
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::from(raw_neuron_vote),
      DecimalNumberWrapper::from(weight),
    )
    .as_tuple()
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
