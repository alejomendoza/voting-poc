use soroban_sdk::{Env, String};
use voting_shared::types::DecimalNumber;

use crate::{
  external_data_provider_contract, AssignedReputationNeuron, AssignedReputationNeuronClient,
};

#[test]
pub fn test_execute() {
  let env = Env::default();

  let assigned_reputation_neuron_id = env.register_contract(None, AssignedReputationNeuron);
  let assigned_reputation_neuron_client =
    AssignedReputationNeuronClient::new(&env, &assigned_reputation_neuron_id);

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();

  assert!(assigned_reputation_neuron_client
    .get_external_data_provider()
    .is_none());
  assigned_reputation_neuron_client.set_external_data_provider(&external_data_provider_id);
  assert!(assigned_reputation_neuron_client
    .get_external_data_provider()
    .is_some());

  let raw_neuron_vote: DecimalNumber = assigned_reputation_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &Some((4, 400)),
  );
  let neuron_vote: DecimalNumber =
    assigned_reputation_neuron_client.weight_function(&raw_neuron_vote);

  assert!(neuron_vote == (1, 320));

  let raw_neuron_vote: DecimalNumber = assigned_reputation_neuron_client.oracle_function(
    &String::from_slice(&env, "user003"),
    &String::from_slice(&env, "project001"),
    &Some((4, 400)),
  );
  let neuron_vote: DecimalNumber =
    assigned_reputation_neuron_client.weight_function(&raw_neuron_vote);

  assert!(neuron_vote == (0, 880));


  assigned_reputation_neuron_client.set_weight(&(2, 500));
  let neuron_vote: DecimalNumber =
    assigned_reputation_neuron_client.weight_function(&raw_neuron_vote);

  assert!(neuron_vote == (2, 200));
}
