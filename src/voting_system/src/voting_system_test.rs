use soroban_sdk::{log, testutils::Logs, Env, String, Vec, Map};

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
