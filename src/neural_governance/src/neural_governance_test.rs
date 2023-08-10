use soroban_sdk::{testutils::Logs, Env, String};

use crate::{
  layer_contract::{self, LayerAggregator},
  simple_neuron_contract, NeuralGovernance, NeuralGovernanceClient,
};

#[test]
pub fn test_setting_layers() {
  let env = Env::default();

  let neural_governance_id = env.register_contract(None, NeuralGovernance);
  let neural_governance_client = NeuralGovernanceClient::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);

  assert!(neural_governance_client.get_layers().is_empty());
  neural_governance_client.add_layer(&layer_id);
  neural_governance_client.add_layer(&layer_id);
  neural_governance_client.add_layer(&layer_id);
  assert!(neural_governance_client.get_layers().len() == 3);
}

#[test]
pub fn test_execute() {
  let env = Env::default();
  env.budget().reset_unlimited();

  let neural_governance_id = env.register_contract(None, NeuralGovernance);
  let neural_governance_client = NeuralGovernanceClient::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_client = layer_contract::Client::new(&env, &layer_id);

  let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  layer_client.set_layer_aggregator(&LayerAggregator::PRODUCT);

  let number_of_neurons = 10;

  for _ in 0..number_of_neurons {
    layer_client.add_neuron(&neuron_id);
  }

  neural_governance_client.add_layer(&layer_id);
  let final_result = neural_governance_client.execute_neural_governance(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
  );
  assert!(final_result == (((2 as u32).pow(number_of_neurons)), 0));
  layer_client.set_layer_aggregator(&LayerAggregator::SUM);
  let final_result = neural_governance_client.execute_neural_governance(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
  );
  assert!(final_result == (2 * number_of_neurons, 0));
  env.logs().print();
}

#[test]
pub fn test_execute_multiple_layers() {
  let env = Env::default();
  env.budget().reset_unlimited();

  let neural_governance_id = env.register_contract(None, NeuralGovernance);
  let neural_governance_client = NeuralGovernanceClient::new(&env, &neural_governance_id);

  let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  let layer_1_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_1_client = layer_contract::Client::new(&env, &layer_1_id);
  layer_1_client.set_layer_aggregator(&LayerAggregator::PRODUCT);
  layer_1_client.add_neuron(&neuron_id);
  layer_1_client.add_neuron(&neuron_id);

  let layer_2_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_2_client = layer_contract::Client::new(&env, &layer_2_id);
  layer_2_client.set_layer_aggregator(&LayerAggregator::SUM);
  layer_2_client.add_neuron(&neuron_id);
  layer_2_client.add_neuron(&neuron_id);

  neural_governance_client.add_layer(&layer_1_id);
  neural_governance_client.add_layer(&layer_2_id);

  let final_result = neural_governance_client.execute_neural_governance(
    &String::from_slice(&env, "user001"),
    &String::from_slice(&env, "project001"),
  );

  assert!(final_result == (20, 400));
}
