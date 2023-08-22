#![allow(non_upper_case_globals)]

use voting_shared::types::VotingSystemError;

use soroban_sdk::{
  contract, contractimpl, contracttype, symbol_short, vec, Address, Env, String, Symbol, Vec,
};

use crate::layer::Layer;

mod template_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_template_neuron.wasm"
  );
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuralGovernance {
  pub layers: Vec<Layer>,
}

impl NeuralGovernance {
  pub fn execute_neural_governance(
    &self,
    env: Env,
    voter_id: String,
    project_id: String,
  ) -> Result<(u32, u32), VotingSystemError> {
    let mut current_layer_result: Option<(u32, u32)> = None;

    if self.layers.is_empty() {
      return Err(VotingSystemError::NoLayersExist);
    }
    for layer in self.layers.clone() {
      let layer_result: Vec<(u32, u32)> =
        layer.execute_layer(env.clone(), voter_id.clone(), project_id.clone(), current_layer_result)?;
      current_layer_result = Some(layer.run_layer_aggregator(env.clone(), layer_result)?);
    }
    current_layer_result.ok_or(VotingSystemError::ResultExpected)
  }
}
