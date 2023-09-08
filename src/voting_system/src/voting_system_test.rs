use crate::{
  decimal_number_wrapper::DecimalNumberWrapper,
  external_data_provider_contract,
  types::{Vote, DEFAULT_WEIGHT},
};
use soroban_sdk::{Env, String};

use crate::{
  layer::{LayerAggregator, NeuronType},
  VotingSystem, VotingSystemClient,
};

#[test]
pub fn test_setting_up_neural_governance() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  assert!(voting_system_client.add_layer() == 0);
  assert!(voting_system_client.add_layer() == 1);
  assert!(voting_system_client.add_layer() == 2);

  assert!(voting_system_client.get_neural_governance().layers.len() == 3);
  voting_system_client.remove_layer(&1);
  assert!(voting_system_client.get_neural_governance().layers.len() == 2);

  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .aggregator
      == LayerAggregator::Unknown
  );
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .aggregator
      == LayerAggregator::Sum
  );

  voting_system_client.add_neuron(&0, &NeuronType::Dummy);
  voting_system_client.add_neuron(&0, &NeuronType::AssignedReputation);
  voting_system_client.add_neuron(&0, &NeuronType::PriorVotingHistory);
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .neurons
      .len()
      == 3
  );
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(1)
      .unwrap()
      .neurons
      .len()
      == 0
  );

  voting_system_client.remove_neuron(&0, &NeuronType::PriorVotingHistory);
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .neurons
      .len()
      == 2
  );

  voting_system_client.set_neuron_weight(&0, &NeuronType::AssignedReputation, &(4, 700));
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .neurons
      .get(NeuronType::Dummy)
      .unwrap()
      == DecimalNumberWrapper::from(DEFAULT_WEIGHT).as_raw()
  );
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .neurons
      .get(NeuronType::AssignedReputation)
      .unwrap()
      == DecimalNumberWrapper::from((4, 700)).as_raw()
  );
}

#[test]
pub fn test_simple_voting() {
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

  let voter_id = String::from_slice(&env, "user001");
  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id, &project_id, &Vote::Yes);

  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (2, 200)
  );
  // change neuron weight
  voting_system_client.set_neuron_weight(&1, &NeuronType::Dummy, &(2, 0));
  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (4, 400)
  );
}

#[test]
pub fn test_assigned_reputation_neuron() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();
  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);

  voting_system_client.add_neuron(&0, &NeuronType::Dummy);
  voting_system_client.add_neuron(&0, &NeuronType::AssignedReputation);

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  let voter_id_1 = String::from_slice(&env, "user001"); // bonus 0,300
  let voter_id_2 = String::from_slice(&env, "user002"); // bonus 0,200
  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::No);

  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (0, 100)
  );

  // change neurons' weights
  voting_system_client.set_neuron_weight(&0, &NeuronType::Dummy, &(2, 0));
  voting_system_client.set_neuron_weight(&0, &NeuronType::AssignedReputation, &(2, 0));

  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (0, 200)
  );
}

#[test]
pub fn test_prior_voting_history_neuron() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();
  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);

  voting_system_client.add_neuron(&0, &NeuronType::PriorVotingHistory);

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  let voter_id_1 = String::from_slice(&env, "user001"); // active rounds: [2, 3], bonusses: [0, 100], [0, 200]
  let voter_id_2 = String::from_slice(&env, "user003"); // active rounds: [2, 3, 4], bonusses: [0, 100], [0, 200], [0, 300]
  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::Yes);

  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (0, 300)
  );
}

#[test]
pub fn test_delegation_more_yes_votes() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::No); // not considered - low rank
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::Yes);

  let consensus = voting_system_client.calculate_quorum_consensus(
    &voter_id_1,
    &voting_system_client
      .get_votes()
      .get(project_id.clone())
      .unwrap(),
  );
  assert!(consensus == Vote::Yes);
}

#[test]
pub fn test_delegation_more_no_votes() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::Yes); // not considered - low rank
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::No);

  let consensus = voting_system_client.calculate_quorum_consensus(
    &voter_id_1,
    &voting_system_client
      .get_votes()
      .get(project_id.clone())
      .unwrap(),
  );
  assert!(consensus == Vote::No);
}

#[test]
pub fn test_delegation_too_many_abstain_votes() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::Yes); // not considered - low rank
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::Abstain);
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::Abstain);
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::No);

  let consensus = voting_system_client.calculate_quorum_consensus(
    &voter_id_1,
    &voting_system_client
      .get_votes()
      .get(project_id.clone())
      .unwrap(),
  );
  assert!(consensus == Vote::Abstain);
}

#[test]
pub fn test_delegation_too_many_delegate_votes() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  // scenario 1 - mote Yes votes
  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::Delegate); // not considered - delegate (this would raise an error when used from `tally` because we would need delegatees for this user as well)
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::Delegate); // not considered - delegate (this would raise an error when used from `tally` because we would need delegatees for this user as well)
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Delegate); // not considered - delegate (this would raise an error when used from `tally` because we would need delegatees for this user as well)
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::No);

  let consensus = voting_system_client.calculate_quorum_consensus(
    &voter_id_1,
    &voting_system_client
      .get_votes()
      .get(project_id.clone())
      .unwrap(),
  );
  assert!(consensus == Vote::Abstain);
}

#[test]
pub fn test_delegation_yes_no_equal() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  // scenario 1 - mote Yes votes
  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::Delegate); // not considered - delegate (this would raise an error when used from `tally` because we would need delegatees for this user as well)
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::Abstain);
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::No);

  let consensus = voting_system_client.calculate_quorum_consensus(
    &voter_id_1,
    &voting_system_client
      .get_votes()
      .get(project_id.clone())
      .unwrap(),
  );
  assert!(consensus == Vote::Abstain);
}

#[test]
pub fn test_delegation_in_practice() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  assert!(voting_system_client.add_layer() == 0);
  voting_system_client.set_layer_aggregator(&0, &LayerAggregator::Sum);
  voting_system_client.add_neuron(&0, &NeuronType::Dummy);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id_1, &project_id, &Vote::Delegate);
  voting_system_client.vote(&voter_id_2, &project_id, &Vote::No);
  voting_system_client.vote(&voter_id_3, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_4, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_5, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_6, &project_id, &Vote::Yes);
  voting_system_client.vote(&voter_id_8, &project_id, &Vote::Abstain);

  let result = voting_system_client
    .tally()
    .get(project_id.clone())
    .unwrap();

  assert!(result == (4, 400));
}
