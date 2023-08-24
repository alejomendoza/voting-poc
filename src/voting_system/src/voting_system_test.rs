use soroban_sdk::{Env, String};
use voting_shared::types::Vote;

use crate::{
  layer::{LayerAggregator, NeuronType},
  VotingSystem, VotingSystemClient,
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

  let voter_id = String::from_slice(&env, "user001");
  let project_id = String::from_slice(&env, "project001");

  voting_system_client.add_project(&project_id);
  voting_system_client.vote(&voter_id, &project_id, &Vote::Yes);

  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (2, 100)
  );
  voting_system_client.set_neuron_weight(&1, &NeuronType::Dummy, &(2, 0));
  assert!(
    voting_system_client
      .tally()
      .get(project_id.clone())
      .unwrap()
      == (4, 200)
  );
}
