#![no_std]
#![allow(non_upper_case_globals)]

// This is a template for any future neurons, it doesn't use macros like unimplemented!() or todo!() so the tests may pass

use soroban_sdk::{contract, contractimpl, Env};
use voting_shared::types::{DecimalNumber, Neuron, ProjectUUID, UserUUID};

#[contract]
pub struct SimpleNeuron;

impl SimpleNeuron {}

#[contractimpl]
impl Neuron for SimpleNeuron {
  fn oracle_function(
    _env: Env,
    _voter_id: UserUUID,
    _project_id: ProjectUUID,
    maybe_previous_layer_vote: Option<DecimalNumber>,
  ) -> DecimalNumber {
    if let Some(previous_layer_vote) = maybe_previous_layer_vote {
      return (previous_layer_vote.0 + 1, previous_layer_vote.1 + 1);
    }
    (1, 0)
  }

  fn weight_function(_env: Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber {
    (raw_neuron_vote.0 * 2, raw_neuron_vote.1 * 2)
  }
}

#[cfg(test)]
mod test;
