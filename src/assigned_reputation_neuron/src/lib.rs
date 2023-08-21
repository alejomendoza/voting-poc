#![no_std]
#![allow(non_upper_case_globals)]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, String};
use voting_shared::{
  decimal_number_wrapper::DecimalNumberWrapper,
  types::{Neuron, VotingSystemError, DEFAULT_WEIGHT},
};

mod external_data_provider_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

// Address of external data provider contract
const EXTERNAL_DATA_PROVIDER: Symbol = symbol_short!("EXTDTPVD");
const WEIGHT: Symbol = symbol_short!("WEIGHT");

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
    voter_id: String,
    _project_id: String,
    maybe_previous_layer_vote: Option<(u32, u32)>,
  ) -> Result<(u32, u32), VotingSystemError> {
    let external_data_provider_id =
      AssignedReputationNeuron::get_external_data_provider(env.clone());
    if external_data_provider_id.is_none() {
      return Err(VotingSystemError::ExternalDataProviderNotSet);
    }
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_id.unwrap());
    let reputation_category = external_data_provider_client.get_user_reputation_category(&voter_id);
    let previous_layer_vote = maybe_previous_layer_vote.unwrap_or((0, 0));
    // todo fixme
    let bonus = external_data_provider_client.get_reputation_score(&reputation_category);
    let res = DecimalNumberWrapper::mul(
      DecimalNumberWrapper::from(previous_layer_vote),
      DecimalNumberWrapper::from(bonus),
    )
    .as_tuple();

    Ok(res)
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
mod assigned_reputation_neuron_test;
