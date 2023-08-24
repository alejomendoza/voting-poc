use soroban_sdk::{log, testutils::Logs, Env, String, Vec, Map};
use voting_shared::types::Vote;

use crate::{
  layer::{Layer, LayerAggregator, NeuronType},
  NeuralGovernance, VotingSystem, VotingSystemClient, decimal_number_wrapper::DecimalNumberWrapper,
};

mod template_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_template_neuron.wasm"
  );
}

mod assigned_reputation_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_assigned_reputation_neuron.wasm"
  );
}

mod prior_voting_history_neuron_contract {
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_prior_voting_history_neuron.wasm"
  );
}

/*
#[test]
pub fn test_one() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();

  let mut ng = voting_system_client.get_neural_governance();
  log!(&env, "-----------------here1", ng.layers);

  ng.layers.push_back(Layer {
    neurons: Map::from_array(&env, [(NeuronType::Dummy, (DecimalNumberWrapper::from((1, 200))).as_raw())]),
    aggregator: LayerAggregator::Sum,
  });
  ng.layers.push_back(Layer {
    neurons: Map::from_array(&env, [(NeuronType::Dummy, (DecimalNumberWrapper::from((0, 875))).as_raw())]),
    aggregator: LayerAggregator::Product,
  });
  // ng.layers.push_back(Layer {
  //   neurons: Vec::from_slice(&env, &[template_neuron_id.clone()]),
  //   aggregator: LayerAggregator::Sum,
  // });
  log!(&env, "-----------------here2", ng.layers);

  // let result = ng.execute_neural_governance(env.clone(), String::from_slice(&env, "user001"), String::from_slice(&env, "project001"));
  // log!(&env, "------------------- result", result);


  env.logs().print();
}
*/

#[test]
pub fn test_the_right_one() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();
  assert!(voting_system_client.add_layer() == 0);
  assert!(voting_system_client.add_layer() == 1);

  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.set_layer_aggregator(&1, &LayerAggregator::Product);

  voting_system_client.add_neuron(&0, &NeuronType::Dummy);
  voting_system_client.add_neuron(&1, &NeuronType::Dummy);

  // voting_system_client.set_neuron_weight(&0, &NeuronType::Dummy, &(4, 200));
  // voting_system_client.set_neuron_weight(&1, &NeuronType::Dummy, &(2, 0));

  let voter_id = String::from_slice(&env, "user001");
  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id, &project_id, &Vote::Yes);

  let ng = voting_system_client.get_neural_governance();
  log!(&env, ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 1", ng.current_layer_id);
  log!(&env, ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 2", ng.layers.len());
  log!(&env, ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 3", ng.layers);

  // let result = voting_system_client.tally();
  // log!(&env, "------------------- result", result);
  assert!(voting_system_client.tally().get(project_id.clone()).unwrap() == (2, 100));
  voting_system_client.set_neuron_weight(&1, &NeuronType::Dummy, &(2, 0));
  assert!(voting_system_client.tally().get(project_id.clone()).unwrap() == (4, 200));

  env.logs().print();
}
