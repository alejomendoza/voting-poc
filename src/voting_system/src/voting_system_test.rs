use soroban_sdk::{log, testutils::Logs, Env, String};
use voting_shared::types::Vote;

use crate::{neural_governance_contract, VotingSystem, VotingSystemClient};

use self::layer_contract::LayerAggregator;

mod simple_neuron_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_simple_neuron.wasm"
  );
}

mod layer_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_layer.wasm"
  );
}

#[test]
#[should_panic]
pub fn test_get_neural_governance_fail() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.get_neural_governance();
}

#[test]
pub fn test_set_neural_governance() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);

  voting_system_client.set_neural_governance(&neural_governance_id);
  voting_system_client.get_neural_governance();
}

#[test]
pub fn test_add_project() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  let neural_governance_client =
    neural_governance_contract::Client::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_client = layer_contract::Client::new(&env, &layer_id);

  let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  layer_client.set_layer_aggregator(&LayerAggregator::SUM);

  layer_client.add_neuron(&neuron_id);

  neural_governance_client.add_layer(&layer_id);

  voting_system_client.set_neural_governance(&neural_governance_id);

  assert!(voting_system_client.get_projects().is_empty());
  assert!(voting_system_client
    .get_projects_current_results()
    .is_empty());
  assert!(voting_system_client.get_projects().is_empty());

  voting_system_client.add_project(&String::from_slice(&env, "project001"));
  voting_system_client.add_project(&String::from_slice(&env, "project002"));
  voting_system_client.add_project(&String::from_slice(&env, "project003"));

  assert!(voting_system_client.get_projects().len() == 3);
}

#[test]
#[should_panic]
pub fn test_add_project_exists_fail() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  let neural_governance_client =
    neural_governance_contract::Client::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_client = layer_contract::Client::new(&env, &layer_id);

  let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  layer_client.set_layer_aggregator(&LayerAggregator::SUM);

  layer_client.add_neuron(&neuron_id);

  neural_governance_client.add_layer(&layer_id);

  voting_system_client.set_neural_governance(&neural_governance_id);

  assert!(voting_system_client.get_projects().is_empty());
  assert!(voting_system_client
    .get_projects_current_results()
    .is_empty());

  voting_system_client.add_project(&String::from_slice(&env, "project001"));
  voting_system_client.add_project(&String::from_slice(&env, "project001"));
}

#[test]
pub fn test_vote() {
  let env = Env::default();
  env.budget().reset_unlimited();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  let neural_governance_client =
    neural_governance_contract::Client::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_client = layer_contract::Client::new(&env, &layer_id);

  let neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  layer_client.set_layer_aggregator(&LayerAggregator::SUM);

  layer_client.add_neuron(&neuron_id);

  neural_governance_client.add_layer(&layer_id);

  voting_system_client.set_neural_governance(&neural_governance_id);

  assert!(voting_system_client.get_projects().is_empty());
  assert!(voting_system_client
    .get_projects_current_results()
    .is_empty());
  assert!(voting_system_client.get_votes().is_empty());

  let project001_id = String::from_slice(&env, "project001");
  let project002_id = String::from_slice(&env, "project002");
  let project003_id = String::from_slice(&env, "project003");

  voting_system_client.add_project(&project001_id);
  voting_system_client.add_project(&project002_id);
  voting_system_client.add_project(&project003_id);
  {
    let user001_id = String::from_slice(&env, "project001");
    voting_system_client.vote(&user001_id, &project001_id, &Vote::YES);
    voting_system_client.vote(&user001_id, &project002_id, &Vote::NO);
    voting_system_client.vote(&user001_id, &project003_id, &Vote::ABSTAIN);
  }
  {
    let user002_id = String::from_slice(&env, "project002");
    voting_system_client.vote(&user002_id, &project001_id, &Vote::YES);
    voting_system_client.vote(&user002_id, &project002_id, &Vote::YES);
    voting_system_client.vote(&user002_id, &project003_id, &Vote::YES);
  }
  {
    let user003_id = String::from_slice(&env, "project003");
    voting_system_client.vote(&user003_id, &project001_id, &Vote::YES);
    voting_system_client.vote(&user003_id, &project002_id, &Vote::YES);
    voting_system_client.vote(&user003_id, &project003_id, &Vote::NO);
  }

  let results = voting_system_client.get_projects_current_results();
  assert!(results.get(project001_id).unwrap() == (6, 0));
  assert!(results.get(project002_id).unwrap() == (2, 0));
  assert!(results.get(project003_id).unwrap() == (0, 0));

  env.logs().print();
}
