use soroban_sdk::{Env, String};
use voting_shared::types::DecimalNumber;

use crate::{TemplateNeuron, TemplateNeuronClient};

#[test]
pub fn test_execute() {
  let env = Env::default();

  let template_neuron_id = env.register_contract(None, TemplateNeuron);
  let template_neuron_client = TemplateNeuronClient::new(&env, &template_neuron_id);

  let raw_neuron_vote: DecimalNumber = template_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &None,
  );
  let neuron_vote: DecimalNumber = template_neuron_client.weight_function(&raw_neuron_vote);

  assert!(neuron_vote == (1, 0));

  let raw_neuron_vote: DecimalNumber = template_neuron_client.oracle_function(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
    &Some(neuron_vote),
  );
  let neuron_vote: DecimalNumber = template_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (2, 100));

  template_neuron_client.set_weight(&(2, 420));
  let neuron_vote: DecimalNumber = template_neuron_client.weight_function(&raw_neuron_vote);
  assert!(neuron_vote == (5, 82));
}