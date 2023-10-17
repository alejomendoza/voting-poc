use crate::types::{DecimalNumber, LayerAggregator, NeuronType, VotingSystemError};
use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{contracttype, Env, Map, String, Vec};

use crate::neurons::{
  assigned_reputation_neuron, dummy_neuron, prior_voting_history_neuron, trust_graph_neuron,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Layer {
  pub id: u32,
  // neuron type, neuron raw weight
  pub neurons: Map<NeuronType, u32>,
  pub aggregator: LayerAggregator,
}

impl Layer {
  pub fn execute_layer(
    &self,
    env: Env,
    voter_id: String,
    _submission_id: String,
    previous_layer_vote: (u32, u32),
  ) -> Result<Vec<(u32, u32)>, VotingSystemError> {
    if self.aggregator == LayerAggregator::Unknown {
      return Err(VotingSystemError::LayerAggregatorNotSet);
    }

    let mut neuron_votes: Vec<(u32, u32)> = Vec::new(&env);
    if self.neurons.is_empty() {
      return Err(VotingSystemError::NoNeuronsExist);
    }
    for (neuron, raw_weight) in self.neurons.iter() {
      let raw_neuron_vote: DecimalNumber = match neuron {
        NeuronType::Dummy => dummy_neuron::oracle_function(env.clone(), voter_id.clone())?,
        NeuronType::AssignedReputation => {
          assigned_reputation_neuron::oracle_function(env.clone(), voter_id.clone())?
        }
        NeuronType::PriorVotingHistory => {
          prior_voting_history_neuron::oracle_function(env.clone(), voter_id.clone())?
        }
        NeuronType::TrustGraph => {
          trust_graph_neuron::oracle_function(env.clone(), voter_id.clone())?
        }
      };
      let neuron_vote = self.run_neuron_weight_function(
        DecimalNumberWrapper::add(
          DecimalNumberWrapper::from(raw_neuron_vote),
          DecimalNumberWrapper::from(previous_layer_vote),
        )
        .as_tuple(),
        DecimalNumberWrapper::from(raw_weight).as_tuple(),
      );
      neuron_votes.push_back(neuron_vote);
    }
    Ok(neuron_votes)
  }

  fn run_neuron_weight_function(
    &self,
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
