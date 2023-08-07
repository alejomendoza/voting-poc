// extern crate std;

// use soroban_sdk::{vec, Env, Vec};
// use voting_shared::types::DecimalNumber;

// use crate::{simple_neuron_contract, Layer, LayerAggregator, LayerClient};

// #[test]
// pub fn test_setting_layer_aggregator() {
//   let env = Env::default();

//   let layer_id = env.register_contract(None, Layer);
//   let layer_client = LayerClient::new(&env, &layer_id);

//   assert!(layer_client.get_layer_aggregator() == LayerAggregator::UNKNOWN);
//   layer_client.set_layer_aggregator(&LayerAggregator::SUM);
//   assert!(layer_client.get_layer_aggregator() == LayerAggregator::SUM);
//   layer_client.set_layer_aggregator(&LayerAggregator::PRODUCT);
//   assert!(layer_client.get_layer_aggregator() == LayerAggregator::PRODUCT);
// }

// #[test]
// pub fn test_setting_neurons() {
//   let env = Env::default();

//   let layer_id = env.register_contract(None, Layer);
//   let layer_client = LayerClient::new(&env, &layer_id);

//   let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

//   assert!(layer_client.get_neurons().is_empty());
//   layer_client.add_neuron(&neuron_id);
//   layer_client.add_neuron(&neuron_id);
//   assert!(layer_client.get_neurons().len() == 2);
// }

// #[test]
// pub fn test_execute() {
//   let env = Env::default();

//   let layer_id = env.register_contract(None, Layer);
//   let layer_client = LayerClient::new(&env, &layer_id);

//   let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

//   layer_client.set_layer_aggregator(&LayerAggregator::SUM);
//   layer_client.add_neuron(&neuron_id);
//   layer_client.add_neuron(&neuron_id);

//   let neuron_votes: Vec<DecimalNumber> = layer_client.execute(&None);

//   assert!(neuron_votes == vec![&env, (2, 0), (2, 0)]);

//   let result: DecimalNumber = layer_client.run_layer_aggregator(&neuron_votes);

//   assert!(result == (4, 0));

//   let neuron_votes: Vec<DecimalNumber> = layer_client.execute(&Some(result));
//   let result: DecimalNumber = layer_client.run_layer_aggregator(&neuron_votes);

//   assert!(result == (20, 4));
// }
