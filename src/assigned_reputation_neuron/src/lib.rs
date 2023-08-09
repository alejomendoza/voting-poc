#![no_std]
#![allow(non_upper_case_globals)]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
use voting_shared::types::{DecimalNumber, Neuron, ProjectUUID, UserUUID, VotingSystemError};

mod external_data_provider_contract {
  use crate::{DecimalNumber, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

// Address of external data provider contract
const EXTERNAL_DATA_PROVIDER: Symbol = symbol_short!("EXTDTPVD");

#[contract]
pub struct AssignedReputationNeuron;

#[contractimpl]
impl AssignedReputationNeuron {
  pub fn set_external_data_provider(env: Env, external_data_provider_address: Address) {
    env
      .storage()
      .instance()
      .set(&EXTERNAL_DATA_PROVIDER, &external_data_provider_address);
  }
  pub fn get_external_data_provider(env: Env) -> Option<Address> {
    env
      .storage()
      .instance()
      .get(&EXTERNAL_DATA_PROVIDER)
      .unwrap_or(None)
  }
}

#[contractimpl]
impl Neuron for AssignedReputationNeuron {
  fn oracle_function(
    env: Env,
    voter_id: UserUUID,
    _project_id: ProjectUUID,
    maybe_previous_layer_vote: Option<DecimalNumber>,
  ) -> Result<DecimalNumber, VotingSystemError> {
    let external_data_provider_id =
      AssignedReputationNeuron::get_external_data_provider(env.clone());
    if external_data_provider_id.is_none() {
      return Err(VotingSystemError::ExternalDataProviderNotSet);
    }
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_id.unwrap());
    let reputation_category = external_data_provider_client.get_user_reputation_category(&voter_id);
    let bonus = match reputation_category {
      external_data_provider_contract::ReputationCategory::Uncategorized
      | external_data_provider_contract::ReputationCategory::Poor => 0,
      other => (other as u32) - 1, // -1 to match with the specification
    };
    let previous_layer_vote = maybe_previous_layer_vote.unwrap_or((0, 0));
    // todo fixme
    Ok((previous_layer_vote.0 * bonus, previous_layer_vote.1))
  }

  fn weight_function(_env: Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber {
    raw_neuron_vote
  }
}

#[cfg(test)]
mod assigned_reputation_neuron_test;
