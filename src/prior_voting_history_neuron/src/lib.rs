#![no_std]
#![allow(non_upper_case_globals)]

use soroban_sdk::{contract, contractimpl, log, symbol_short, Address, Env, String, Symbol};
use voting_shared::{
  decimal_number_persist::DecimalNumberPersist,
  types::{DecimalNumber, Neuron, ProjectUUID, UserUUID},
};

mod external_data_provider_contract {
  use crate::DecimalNumber;
  use crate::UserUUID;
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

const EXTERNAL_DATA_PROVIDER: Symbol = symbol_short!("EXTDTPVD");

#[contract]
pub struct PriorVotingHistoryNeuron;

#[contractimpl]
impl PriorVotingHistoryNeuron {
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
impl Neuron for PriorVotingHistoryNeuron {
  fn oracle_function(
    env: Env,
    voter_id: UserUUID,
    _project_id: ProjectUUID,
    maybe_previous_layer_vote: Option<DecimalNumber>,
  ) -> DecimalNumber {
    let external_data_provider_id =
      PriorVotingHistoryNeuron::get_external_data_provider(env.clone());
    let external_data_provider_client =
      external_data_provider_contract::Client::new(&env, &external_data_provider_id.unwrap());
    // todo improve this code pls
    let voter_active_rounds =
      external_data_provider_client.get_user_prior_voting_history(&voter_id);
    let round_bonus_map = external_data_provider_client.get_round_bonus_map();
    let previous_layer_vote = maybe_previous_layer_vote.unwrap_or((0, 0));
    let previous_layer_vote: DecimalNumberPersist = DecimalNumberPersist::from(previous_layer_vote);
    let mut bonus_result = DecimalNumberPersist::from(previous_layer_vote.as_tuple());
    for round in voter_active_rounds {
      let bonus: DecimalNumber = round_bonus_map
        .get(round)
        .expect("given round not found in round bonus map");
      bonus_result = DecimalNumberPersist::mul(
        DecimalNumberPersist::from(bonus_result),
        DecimalNumberPersist::from(bonus),
      );
    }
    bonus_result = DecimalNumberPersist::add(previous_layer_vote, bonus_result);
    bonus_result.as_tuple()
  }

  fn weight_function(_env: Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber {
    raw_neuron_vote
  }
}

#[cfg(test)]
mod test;
