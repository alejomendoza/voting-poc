use soroban_sdk::{Env, String, Vec};

use crate::neuron::Neuron;
use crate::types::DecimalNumber;

pub struct Layer<'a> {
  pub neurons: &'a [&'a dyn Neuron],
  pub layer_aggregator: &'a dyn Fn(Vec<DecimalNumber>) -> DecimalNumber,
}

impl Layer<'_> {
  pub fn execute(
    &self,
    env: &Env,
    previous_layer_vote: Option<DecimalNumber>,
  ) -> Vec<DecimalNumber> {
    let mut neuron_votes = Vec::new(&env);
    if self.neurons.is_empty() {
      panic!("no neurons detected");
    }
    for neuron in self.neurons.iter() {
      let raw_neuron_vote = neuron.oracle_function(
        // TODO ARGS
        &env,
        String::from_slice(&env, "voter id"),
        String::from_slice(&env, "project id"),
        &previous_layer_vote,
      );
      neuron_votes.push_back(neuron.weight_function(&env, raw_neuron_vote));
    }
    neuron_votes
  }
}
