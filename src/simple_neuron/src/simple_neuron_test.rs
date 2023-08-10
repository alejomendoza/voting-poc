use soroban_sdk::{Env, String};
use voting_shared::types::DecimalNumber;

use crate::{SimpleNeuron, SimpleNeuronClient};

#[test]
pub fn test_execute() {
  let env = Env::default();

  let simple_neuron_id = env.register_contract(None, SimpleNeuron);
  let simple_neuron_client = SimpleNeuronClient::new(&env, &simple_neuron_id);

  let raw_neuron_vote: DecimalNumber = simple_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &None,
  );
  let neuron_vote: DecimalNumber = simple_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (2, 0));

  let raw_neuron_vote: DecimalNumber = simple_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &Some(neuron_vote),
  );
  let neuron_vote: DecimalNumber = simple_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (6, 200));
}
