use crate::{
  external_data_provider_contract,
  types::{LayerAggregator, NeuronType, Vote, DEFAULT_WEIGHT},
};
use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{vec, Env, Map, String, Vec};

use crate::{VotingSystem, VotingSystemClient};

fn initialize_voting_system(env: &Env) -> VotingSystemClient {
  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);
  voting_system_client.initialize();

  voting_system_client
}

fn initialize_external_data_provider<'a>(
  env: &Env,
  voting_system_client: &VotingSystemClient,
) -> external_data_provider_contract::Client<'a> {
  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

  external_data_provider_client
}

#[test]
pub fn test_setting_up_neural_governance() {
  let env = Env::default();

  let voting_system_client = initialize_voting_system(&env);

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
pub fn test_setting_up_neural_governance_batch() {
  let env = Env::default();

  let voting_system_client = initialize_voting_system(&env);

  let mut neurons: Vec<(String, u32)> = Vec::new(&env);
  neurons.push_back((String::from_slice(&env, "TrustGraph"), 0));
  neurons.push_back((String::from_slice(&env, "Dummy"), 1100));

  voting_system_client.setup_layer(&String::from_slice(&env, "Sum"), &neurons);

  let mut neurons: Vec<(String, u32)> = Vec::new(&env);
  neurons.push_back((String::from_slice(&env, "AssignedReputation"), 2000));
  neurons.push_back((String::from_slice(&env, "PriorVotingHistory"), 3000));

  voting_system_client.setup_layer(&String::from_slice(&env, "Product"), &neurons);

  let neural_governance = voting_system_client.get_neural_governance();

  assert!(neural_governance.layers.len() == 2);

  let layer0 = neural_governance.layers.get(0).unwrap();
  let layer1 = neural_governance.layers.get(1).unwrap();

  assert!(
    layer0.aggregator == LayerAggregator::Sum || layer0.aggregator == LayerAggregator::Product
  );
  assert!(
    layer1.aggregator == LayerAggregator::Sum || layer1.aggregator == LayerAggregator::Product
  );
  assert!(layer0.aggregator != layer1.aggregator);

  if layer0.aggregator == LayerAggregator::Sum {
    assert!(
      layer0.neurons.get(NeuronType::TrustGraph).unwrap()
        == DecimalNumberWrapper::from(DEFAULT_WEIGHT).as_raw()
    );
    assert!(layer0.neurons.get(NeuronType::Dummy).unwrap() == 1100);

    assert!(layer1.neurons.get(NeuronType::AssignedReputation).unwrap() == 2000);
    assert!(layer1.neurons.get(NeuronType::PriorVotingHistory).unwrap() == 3000);
  } else {
    assert!(
      layer1.neurons.get(NeuronType::TrustGraph).unwrap()
        == DecimalNumberWrapper::from(DEFAULT_WEIGHT).as_raw()
    );
    assert!(layer1.neurons.get(NeuronType::Dummy).unwrap() == 1100);

    assert!(layer0.neurons.get(NeuronType::AssignedReputation).unwrap() == 2000);
    assert!(layer0.neurons.get(NeuronType::PriorVotingHistory).unwrap() == 3000);
  }
}

#[test]
pub fn test_simple_voting() {
  let env = Env::default();

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);
  voting_system_client.get_external_data_provider();

  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();
  voting_system_client.set_external_data_provider(&external_data_provider_id);

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
  env.budget().reset_unlimited();

  let voting_system_client = initialize_voting_system(&env);

  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "Dummy"));
  voting_system_client.add_neuron(&0, &String::from_slice(&env, "AssignedReputation"));

  initialize_external_data_provider(&env, &voting_system_client);

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
  env.budget().reset_unlimited();

  let voting_system_client = initialize_voting_system(&env);

  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "PriorVotingHistory"));

  initialize_external_data_provider(&env, &voting_system_client);

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
  env.budget().reset_unlimited();

  let voting_system_client = initialize_voting_system(&env);

  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "TrustGraph"));

  let external_data_provider_client =
    initialize_external_data_provider(&env, &voting_system_client);

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

  let calculated = external_data_provider_client.calculate_page_rank();

  assert!(
    calculated
      == Map::from_array(
        &env,
        [
          (String::from_slice(&env, "user001"), (0, 338)),
          (String::from_slice(&env, "user002"), (0, 260)),
          (String::from_slice(&env, "user003"), (0, 190)),
          (String::from_slice(&env, "user004"), (0, 180)),
        ]
      )
  );

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (0, 78)
  );
}

#[test]
pub fn test_graph_bonus_2() {
  let env = Env::default();
  env.budget().reset_unlimited();

  let voting_system_client = initialize_voting_system(&env);

  assert!(voting_system_client.add_layer() == 0);

  voting_system_client.set_layer_aggregator(&0, &String::from_slice(&env, "Sum"));

  voting_system_client.add_neuron(&0, &String::from_slice(&env, "TrustGraph"));

  let external_data_provider_client =
    initialize_external_data_provider(&env, &voting_system_client);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");
  let voter_id_3 = String::from_slice(&env, "user003");
  let voter_id_4 = String::from_slice(&env, "user004");
  let voter_id_5 = String::from_slice(&env, "user005");

  let mut new_trust_map: Map<String, Map<String, ()>> = Map::new(&env);

  new_trust_map.set(
    voter_id_1.clone(),
    Map::from_array(
      &env,
      [
        (voter_id_2.clone(), ()),
        (voter_id_3.clone(), ()),
        (voter_id_4.clone(), ()),
        (voter_id_5.clone(), ()),
      ],
    ),
  );

  new_trust_map.set(
    voter_id_2.clone(),
    Map::from_array(
      &env,
      [
        (voter_id_3.clone(), ()),
        (voter_id_4.clone(), ()),
        (voter_id_5.clone(), ()),
      ],
    ),
  );

  new_trust_map.set(
    voter_id_3.clone(),
    Map::from_array(&env, [(voter_id_4.clone(), ()), (voter_id_5.clone(), ())]),
  );

  new_trust_map.set(
    voter_id_4.clone(),
    Map::from_array(&env, [(voter_id_5.clone(), ())]),
  );

  external_data_provider_client.set_trust_map(&new_trust_map);

  let submission_id = String::from_slice(&env, "submission001");

  voting_system_client.add_submission(&submission_id);
  voting_system_client.vote(&voter_id_1, &submission_id, &String::from_slice(&env, "No"));
  voting_system_client.vote(
    &voter_id_2,
    &submission_id,
    &String::from_slice(&env, "Yes"),
  );

  let calculated = external_data_provider_client.calculate_page_rank();

  assert!(
    calculated
      == Map::from_array(
        &env,
        [
          (String::from_slice(&env, "user001"), (0, 30)),
          (String::from_slice(&env, "user002"), (0, 36)),
          (String::from_slice(&env, "user003"), (0, 46)),
          (String::from_slice(&env, "user004"), (0, 65)),
          (String::from_slice(&env, "user005"), (0, 120)),
        ]
      )
  );

  assert!(
    voting_system_client
      .tally()
      .get(submission_id.clone())
      .unwrap()
      == (0, 6)
  );
}

#[test]
pub fn test_delegation_more_yes_votes() {
  let env = Env::default();

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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
  assert!(voting_system_client.get_delegatees().len() == 1);
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

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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

  let voting_system_client = initialize_voting_system(&env);

  initialize_external_data_provider(&env, &voting_system_client);

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

  let voting_system_client = initialize_voting_system(&env);

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
  assert!(voting_system_client.get_votes_length() == 3);
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
  // test multiple_vote_operations_vec
  let current_user_votes = voting_system_client.multiple_vote_operations_vec(
    &voter_id,
    &Vec::from_array(
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

#[test]
pub fn test_decomposed_tally() {
  let env = Env::default();
  env.budget().reset_unlimited();

  let voting_system_client = initialize_voting_system(&env);

  let voter_id_1 = String::from_slice(&env, "user001");
  let voter_id_2 = String::from_slice(&env, "user002");

  let submission_1_id = String::from_slice(&env, "submission001");
  let submission_2_id = String::from_slice(&env, "submission002");

  voting_system_client.vote(
    &voter_id_1,
    &submission_1_id,
    &String::from_slice(&env, "Yes"),
  );

  voting_system_client.vote(
    &voter_id_2,
    &submission_1_id,
    &String::from_slice(&env, "No"),
  );

  voting_system_client.vote(
    &voter_id_2,
    &submission_2_id,
    &String::from_slice(&env, "Yes"),
  );

  let n_layers = 5;

  for i in 0..n_layers {
    assert!(voting_system_client.add_layer() == i);
    if i % 2 == 0 {
      voting_system_client.set_layer_aggregator(&i, &String::from_slice(&env, "Sum"));
    } else {
      voting_system_client.set_layer_aggregator(&i, &String::from_slice(&env, "Product"));
    }

    voting_system_client.add_neuron(&i, &String::from_slice(&env, "TrustGraph"));
    voting_system_client.add_neuron(&i, &String::from_slice(&env, "AssignedReputation"));
    voting_system_client.add_neuron(&i, &String::from_slice(&env, "PriorVotingHistory"));
  }

  initialize_external_data_provider(&env, &voting_system_client);

  voting_system_client.calculate_page_rank();

  let normalized_votes: Map<String, Map<String, String>> = voting_system_client.normalize_votes();
  let mut voters_voting_powers: Map<String, u32> = Map::new(&env);
  for (submission_id, submission_votes) in normalized_votes.clone() {
    for (voter_id, _normalized_vote) in submission_votes {
      if voters_voting_powers.get(voter_id.clone()).is_none() {
        let voting_power = voting_system_client.voting_power_for_voter(&voter_id, &submission_id);
        voters_voting_powers.set(voter_id, DecimalNumberWrapper::from(voting_power).as_raw());
      }
    }
  }

  // test normalize_votes_for_submission
  let submission_1_normalized_votes =
    voting_system_client.normalize_votes_for_submission(&submission_1_id);
  assert!(submission_1_normalized_votes.len() == 2);

  let final_voting_powers =
    voting_system_client.submissions_voting_powers(&voters_voting_powers, &normalized_votes);

  let fetched_voting_powers = voting_system_client.get_voting_powers();

  assert!(fetched_voting_powers.get(voter_id_1.clone()).unwrap() == (1080, 416));
  assert!(fetched_voting_powers.get(voter_id_2.clone()).unwrap() == (6, 490));

  voting_system_client.set_voting_power_for_user(&voter_id_1.clone(), &(1, 0));

  assert!(
    voting_system_client
      .get_voting_powers()
      .get(voter_id_1.clone())
      .unwrap()
      == (1, 0)
  );

  assert!(final_voting_powers.get(submission_1_id).unwrap() == (1073, 926));
  assert!(final_voting_powers.get(submission_2_id).unwrap() == (6, 490));

  let mut voters_voting_powers_vec: Vec<(String, u32)> = Vec::new(&env);
  let mut normalized_votes_vec: Vec<(String, String, String)> = Vec::new(&env);

  for (voter_id, power) in voters_voting_powers {
    voters_voting_powers_vec.push_back((voter_id, power));
  }

  for (submission_id, normalized_votes_map) in normalized_votes {
    for (voter_id, normalized_vote) in normalized_votes_map {
      normalized_votes_vec.push_back((submission_id.clone(), voter_id, normalized_vote));
    }
  }

  let final_voting_powers_from_vec = voting_system_client
    .submissions_voting_powers_vec(&voters_voting_powers_vec, &normalized_votes_vec);

  assert!(final_voting_powers == final_voting_powers_from_vec);
}
