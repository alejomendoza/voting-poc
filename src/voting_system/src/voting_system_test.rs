use crate::{
  external_data_provider_contract,
  page_rank::Rank,
  types::{LayerAggregator, NeuronType, Vote, DEFAULT_WEIGHT},
};
use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{vec, Env, Map, String};

use crate::{VotingSystem, VotingSystemClient};

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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  assert!(
    voting_system_client
      .get_neural_governance()
      .layers
      .get(0)
      .unwrap()
      .aggregator
      == LayerAggregator::Sum
  );

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "AssignedReputation"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "PriorVotingHistory"));
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

  voting_system_client.remove_neuron(&0, &String::from_slice(&env, "PriorVotingHistory"));
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

  voting_system_client.set_neuron_weight(
    &0,
    &String::from_slice(&env, "AssignedReputation"),
    &4700,
  );
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
      == DecimalNumberWrapper::from("4.7").as_raw()
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

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.set_layer_aggregator(&1, &String::from_slice(&env, "Product"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));
  voting_system_client.add_neuron(&1, &String::from_slice(&env, "Dummy"));

  let voter_id = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let submission_id = String::from_slice(&env, "submission001");
  let submission_id_2 = String::from_slice(&env, "submission002");

  assert!(voting_system_client.get_submissions().is_empty());
  voting_system_client.add_submission(&submission_id_2);
  assert!(voting_system_client.get_submissions().len() == 1);

  assert!(voting_system_client.get_voters().is_empty());
  let current_user_votes =
    voting_system_client.vote(&voter_id, &submission_id, &String::from_slice(&env, "No"));
  assert!(current_user_votes.len() == 1);
  assert!(voting_system_client.get_submissions().len() == 2);
  // test overriding the vote
  let current_user_votes =
    voting_system_client.vote(&voter_id, &submission_id, &String::from_slice(&env, "Yes"));
  assert!(current_user_votes.len() == 1);

  let current_user_votes = voting_system_client.vote(
    &voter_id,
    &submission_id_2,
    &String::from_slice(&env, "Yes"),
  );
  assert!(current_user_votes.len() == 2);

  assert!(voting_system_client.get_voters().len() == 1);
  let current_user_votes = voting_system_client.vote(
    &voter_id_2,
    &submission_id_2,
    &String::from_slice(&env, "Yes"),
  );
  assert!(current_user_votes.len() == 1);
  assert!(voting_system_client.get_voters().len() == 2);
  let current_user_votes = voting_system_client.remove_vote(&voter_id_2, &submission_id_2);
  assert!(current_user_votes.len() == 0);
  assert!(voting_system_client.get_voters().len() == 1);

  voting_system_client.vote(
    &voter_id,
    &submission_id_2,
    &String::from_slice(&env, "Yes"),
  );
  assert!(voting_system_client.get_votes_for_user(&voter_id).len() == 2);

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (2, 200)
  );
  // change neuron weight
  voting_system_client.set_neuron_weight(&1, &String::from_slice(&env, "Dummy"), &2000);
  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
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

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "AssignedReputation"));

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  let voter_id_1 = String::from_slice(&env, "user001"); // bonus 0,300
  let voter_id_2 = String::from_slice(&env, "user002"); // bonus 0,200
  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  voting_system_client.vote(
    &voter_id_1,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_2, &submission_id, &String::from_slice(&env, "No"));

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (0, 100)
  );

  // change neurons' weights
  voting_system_client.set_neuron_weight(&0, &String::from_slice(&env, "Dummy"), &2000);
  voting_system_client.set_neuron_weight(
    &0,
    &String::from_slice(&env, "AssignedReputation"),
    &2000,
  );

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
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

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "PriorVotingHistory"));

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  let voter_id_1 = String::from_slice(&env, "user001"); // active rounds: [2, 3], bonusses: [0, 100], [0, 200]
  let voter_id_2 = String::from_slice(&env, "user003"); // active rounds: [2, 3, 4], bonusses: [0, 100], [0, 200], [0, 300]
  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  voting_system_client.vote(&voter_id_1, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (0, 300)
  );
}

#[test]
pub fn test_graph_bonus() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.initialize();
  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "TrustGraph"));

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  voting_system_client.vote(
    &voter_id_1,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_2, &submission_id, &String::from_slice(&env, "No"));

  let tm = external_data_provider_client.get_trust_map();
  let rank = Rank::from_pages(&env, tm);
  let calculated = rank.calculate(&env);

  assert!(
    calculated
      == Map::from_array(
        &env,
        [
          (String::from_slice(&env, "user001"), (0, 344)),
          (String::from_slice(&env, "user002"), (0, 185)),
          (String::from_slice(&env, "user003"), (0, 339)),
          (String::from_slice(&env, "user004"), (0, 181)),
        ]
      )
  );

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (0, 159)
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(&voter_id_2, &submission_id, &String::from_slice(&env, "No")); // not considered - low rank
  voting_system_client.vote(&voter_id_3, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(&voter_id_4, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(
    &voter_id_5,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_6,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_8,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );

  let consensus =
    voting_system_client.calculate_quorum_consensus(&voter_id_1, &submission_id.clone());
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  ); // not considered - low rank
  voting_system_client.vote(
    &voter_id_3,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_4, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(
    &voter_id_5,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_6, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(&voter_id_8, &submission_id, &String::from_slice(&env, "No"));

  let consensus =
    voting_system_client.calculate_quorum_consensus(&voter_id_1, &submission_id.clone());
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  ); // not considered - low rank
  voting_system_client.vote(
    &voter_id_3,
    &submission_id,
    &String::from_slice(&env, "Abstain"),
  );
  voting_system_client.vote(
    &voter_id_4,
    &submission_id,
    &String::from_slice(&env, "Abstain"),
  );
  voting_system_client.vote(
    &voter_id_5,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_6, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(&voter_id_8, &submission_id, &String::from_slice(&env, "No"));

  let consensus =
    voting_system_client.calculate_quorum_consensus(&voter_id_1, &submission_id.clone());
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let voter_id_999 = String::from_slice(&env, "user999");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.delegate(
    &voter_id_3,
    &submission_id,
    &vec![
      &env,
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
    ],
  );
  voting_system_client.delegate(
    &voter_id_4,
    &submission_id,
    &vec![
      &env,
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
    ],
  );
  voting_system_client.delegate(
    &voter_id_5,
    &submission_id,
    &vec![
      &env,
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
    ],
  );
  voting_system_client.vote(&voter_id_6, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(&voter_id_8, &submission_id, &String::from_slice(&env, "No"));

  let consensus =
    voting_system_client.calculate_quorum_consensus(&voter_id_1, &submission_id.clone());
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let voter_id_999 = String::from_slice(&env, "user999");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.delegate(
    &voter_id_3,
    &submission_id,
    &vec![
      &env,
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
      voter_id_999.clone(),
    ],
  );
  voting_system_client.vote(
    &voter_id_4,
    &submission_id,
    &String::from_slice(&env, "Abstain"),
  );
  voting_system_client.vote(
    &voter_id_5,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(&voter_id_6, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(&voter_id_8, &submission_id, &String::from_slice(&env, "No"));

  let consensus =
    voting_system_client.calculate_quorum_consensus(&voter_id_1, &submission_id.clone());
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
  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");
  let voter_id_6 = String::from_slice(&env, "user006");
  let voter_id_8 = String::from_slice(&env, "user008");

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  let delegatees = vec![
    &env,
    voter_id_2.clone(),
    voter_id_3.clone(),
    voter_id_4.clone(),
    voter_id_5.clone(),
    voter_id_6.clone(),
    voter_id_8.clone(),
  ];
  voting_system_client.delegate(&voter_id_1, &submission_id, &delegatees);
  voting_system_client.vote(&voter_id_2, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(
    &voter_id_3,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_4,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_5,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_6,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );
  voting_system_client.vote(
    &voter_id_8,
    &submission_id,
    &String::from_slice(&env, "Abstain"),
  );

  let result = voting_system_client
    .tally()
    .get(submission_id.clone())
    .unwrap();

  assert!(result == (4, 400));
}

#[test]
pub fn test_multiple_voting_operations() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  assert!(voting_system_client.add_layer() == 0);
  assert!(voting_system_client.add_layer() == 1);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));
  voting_system_client.set_layer_aggregator(&1, &String::from_slice(&env, "Product"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));
  voting_system_client.add_neuron(&1, &String::from_slice(&env, "Dummy"));

  let voter_id = String::from_slice(&env, "user001");
  let submission_id = String::from_slice(&env, "submission001");
  let submission_id_2 = String::from_slice(&env, "submission002");
  let submission_id_3 = String::from_slice(&env, "submission003");
  let current_user_votes = voting_system_client.multiple_vote_operations(
    &voter_id,
    &Map::from_array(
      &env,
      [
        (submission_id.clone(), String::from_slice(&env, "No")),
        (submission_id_2.clone(), String::from_slice(&env, "Yes")),
        (submission_id_3.clone(), String::from_slice(&env, "Yes")),
      ],
    ),
  );

  assert!(current_user_votes.len() == 3);
  assert!(voting_system_client.get_voters().len() == 1);
  let votes = voting_system_client.get_votes();
  assert!(votes.len() == 3);
  assert!(
    votes
      .get(submission_id.clone())
      .unwrap()
      .get(voter_id.clone())
      .unwrap()
      == Vote::No
  );
  assert!(
    votes
      .get(submission_id_2.clone())
      .unwrap()
      .get(voter_id.clone())
      .unwrap()
      == Vote::Yes
  );
  assert!(
    votes
      .get(submission_id_3.clone())
      .unwrap()
      .get(voter_id.clone())
      .unwrap()
      == Vote::Yes
  );

  //
  let current_user_votes = voting_system_client.multiple_vote_operations(
    &voter_id,
    &Map::from_array(
      &env,
      [
        (submission_id.clone(), String::from_slice(&env, "Remove")),
        (submission_id_2.clone(), String::from_slice(&env, "Remove")),
        (submission_id_3.clone(), String::from_slice(&env, "No")),
      ],
    ),
  );
  assert!(current_user_votes.len() == 1);
  assert!(voting_system_client.get_voters().len() == 1);
  let votes = voting_system_client.get_votes();

  assert!(votes.len() == 1);
  assert!(
    votes
      .get(submission_id_3.clone())
      .unwrap()
      .get(voter_id.clone())
      .unwrap()
      == Vote::No
  );

  let current_user_votes = voting_system_client.multiple_vote_operations(
    &voter_id,
    &Map::from_array(
      &env,
      [(submission_id_3.clone(), String::from_slice(&env, "Remove"))],
    ),
  );
  assert!(current_user_votes.is_empty());
  assert!(voting_system_client.get_voters().is_empty());
}
