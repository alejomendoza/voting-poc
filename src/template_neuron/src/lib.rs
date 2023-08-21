#![no_std]
#![allow(non_upper_case_globals)]

// This is a template for any future neurons, it doesn't use macros like unimplemented!() or todo!() so the tests may pass

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, String};
use voting_shared::{
  decimal_number_wrapper::DecimalNumberWrapper,
  types::{Neuron, VotingSystemError, DEFAULT_WEIGHT},
};

const WEIGHT: Symbol = symbol_short!("WEIGHT");

#[contract]
pub struct TemplateNeuron;

#[contractimpl]
impl TemplateNeuron {}

#[contractimpl]
impl Neuron for TemplateNeuron {
  fn oracle_function(
    _env: Env,
    _voter_id: String,
    _project_id: String,
    maybe_previous_layer_vote: Option<(u32, u32)>,
  ) -> Result<(u32, u32), VotingSystemError> {
    if let Some(previous_layer_vote) = maybe_previous_layer_vote {
      return Ok((previous_layer_vote.0 + 1, previous_layer_vote.1 + 100));
    }
    Ok((1, 0))
  }

  fn weight_function(env: Env, raw_neuron_vote: (u32, u32)) -> (u32, u32) {
    let weight: (u32, u32) = env
      .storage()
      .instance()
      .get(&WEIGHT)
      .unwrap_or(DEFAULT_WEIGHT);

    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::from(raw_neuron_vote),
      DecimalNumberWrapper::from(weight),
    )
    .as_tuple()
  }

  fn set_weight(env: Env, new_weight: (u32, u32)) {
    env
      .storage()
      .instance()
      .set(&WEIGHT, &DecimalNumberWrapper::from(new_weight).as_tuple());
  }
}

#[cfg(test)]
mod template_neuron_test;
