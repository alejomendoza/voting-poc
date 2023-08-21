#![no_std]
#![allow(non_upper_case_globals)]

use voting_shared::types::VotingSystemError;

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec, String};

mod template_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_template_neuron.wasm"
  );
}

mod layer_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_layer.wasm"
  );
}

// Vec<Address> of layers contracts
const LAYERS: Symbol = symbol_short!("LAYERS");

#[contract]
pub struct NeuralGovernance;

#[contractimpl]
impl NeuralGovernance {
  pub fn execute_neural_governance(
    env: Env,
    voter_id: String,
    project_id: String,
  ) -> Result<(u32, u32), VotingSystemError> {
    let mut current_layer_result: Option<(u32, u32)> = None;

    let layers: Vec<Address> = NeuralGovernance::get_layers(env.clone());
    if layers.is_empty() {
      return Err(VotingSystemError::NoLayersExist);
    }
    for layer in layers {
      let layer_client = layer_contract::Client::new(&env, &layer);
      let layer_result: Vec<(u32, u32)> =
        layer_client.execute_layer(&voter_id, &project_id, &current_layer_result);
      current_layer_result = Some(layer_client.run_layer_aggregator(&layer_result));
    }
    current_layer_result.ok_or(VotingSystemError::ResultExpected)
  }

  pub fn add_layer(env: Env, layer_address: Address) {
    let mut layers: Vec<Address> = NeuralGovernance::get_layers(env.clone());

    layers.push_back(layer_address);

    env.storage().instance().set(&LAYERS, &layers);
  }

  pub fn get_layers(env: Env) -> Vec<Address> {
    env.storage().instance().get(&LAYERS).unwrap_or(vec![&env])
  }
}

#[cfg(test)]
mod neural_governance_test;
