use soroban_sdk::{log, testutils::Logs, Env, String};
use voting_shared::types::DecimalNumber;

use crate::{
  external_data_provider_contract, PriorVotingHistoryNeuron, PriorVotingHistoryNeuronClient,
};

#[test]
pub fn test_execute() {
  let env = Env::default();

  let prior_voting_history_neuron_id = env.register_contract(None, PriorVotingHistoryNeuron);
  let prior_voting_history_neuron_client =
    PriorVotingHistoryNeuronClient::new(&env, &prior_voting_history_neuron_id);

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();

  prior_voting_history_neuron_client.set_external_data_provider(&external_data_provider_id);
  let raw_neuron_vote: DecimalNumber = prior_voting_history_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &Some((1, 100)),
  );
  let neuron_vote: DecimalNumber =
    prior_voting_history_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (1, 122));

  let raw_neuron_vote: DecimalNumber = prior_voting_history_neuron_client.oracle_function(
    &String::from_slice(&env, "user003"),
    &String::from_slice(&env, "project001"),
    &Some((5, 49)),
  );
  let neuron_vote: DecimalNumber =
    prior_voting_history_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (5, 79));
}
