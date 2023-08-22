use soroban_sdk::{log, testutils::Logs, Env, String, Vec};

use crate::{
  layer::{Layer, LayerAggregator},
  NeuralGovernance, VotingSystem, VotingSystemClient,
};

#[test]
pub fn test_one() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();

  log!(
    &env,
    "-----------------here1",
    voting_system_client.get_layers()
  );
  let mut ng = voting_system_client.get_neural_governance();
  log!(&env, "-----------------here2", ng.layers.len());
  log!(&env, "-----------------here3", ng.layers);
  ng.layers.push_back(Layer {
    neurons: Vec::new(&env),
    aggregator: LayerAggregator::Sum,
  });
  log!(&env, "-----------------here4", ng.layers.len());
  log!(&env, "-----------------here5", ng.layers);
  // NeuralGovernance::add_layer(env.clone(), Layer {
  //   neurons: Vec::new(&env),
  //   aggregator: LayerAggregator::Sum,
  // });
  // log!(&env, "-----------------here3", NeuralGovernance::get_layers(env.clone()));

  env.logs().print();
}
