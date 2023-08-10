use soroban_sdk::{Env, String};
use voting_shared::types::{Vote, VotingSystemError};

use crate::{neural_governance_contract, VotingSystem, VotingSystemClient};

use self::layer_contract::LayerAggregator;

mod external_data_provider_contract {
  use crate::{DecimalNumber, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_external_data_provider.wasm"
  );
}

mod simple_neuron_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_simple_neuron.wasm"
  );
}

mod assigned_reputation_neuron_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_assigned_reputation_neuron.wasm"
  );
}

mod prior_voting_history_neuron_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_prior_voting_history_neuron.wasm"
  );
}

mod layer_contract {
  use crate::{DecimalNumber, ProjectUUID, UserUUID};
  soroban_sdk::contractimport!(
    file = "../../target/wasm32-unknown-unknown/release/voting_layer.wasm"
  );
}

#[test]
pub fn test_set_neural_governance() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  assert!(voting_system_client.try_get_neural_governance().is_err());

  voting_system_client.set_neural_governance(&neural_governance_id);
  voting_system_client.get_neural_governance();
}

#[test]
pub fn test_add_project() {
  let env = Env::default();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  voting_system_client.add_project(&String::from_slice(&env, "project001"));
  voting_system_client.add_project(&String::from_slice(&env, "project002"));
  voting_system_client.add_project(&String::from_slice(&env, "project003"));

  assert!(voting_system_client.get_projects().len() == 3);

  assert!(voting_system_client
    .try_add_project(&String::from_slice(&env, "project001"))
    .is_err());
}

#[test]
pub fn test_vote() -> Result<(), VotingSystemError> {
  let env = Env::default();
  env.budget().reset_unlimited();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  let neural_governance_client =
    neural_governance_contract::Client::new(&env, &neural_governance_id);

  let layer_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_client = layer_contract::Client::new(&env, &layer_id);

  let simple_neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

  layer_client.set_layer_aggregator(&LayerAggregator::SUM);

  layer_client.add_neuron(&simple_neuron_id);

  neural_governance_client.add_layer(&layer_id);

  voting_system_client.set_neural_governance(&neural_governance_id);

  assert!(voting_system_client.get_projects().is_empty());
  assert!(voting_system_client.tally().is_empty());
  assert!(voting_system_client.get_votes().is_empty());

  let project001_id = String::from_slice(&env, "project001");
  let project002_id = String::from_slice(&env, "project002");
  let project003_id = String::from_slice(&env, "project003");

  voting_system_client.add_project(&project001_id);
  voting_system_client.add_project(&project002_id);
  voting_system_client.add_project(&project003_id);
  {
    let user001_id = String::from_slice(&env, "user001");
    let _ = voting_system_client
      .try_vote(&user001_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user001_id, &project002_id, &Vote::NO)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user001_id, &project003_id, &Vote::ABSTAIN)
      .map_err(|err| err.unwrap())?;
  }
  {
    let user002_id = String::from_slice(&env, "user002");
    let _ = voting_system_client
      .try_vote(&user002_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user002_id, &project002_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user002_id, &project003_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
  }
  {
    let user003_id = String::from_slice(&env, "user003");
    let _ = voting_system_client
      .try_vote(&user003_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user003_id, &project002_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user003_id, &project003_id, &Vote::NO)
      .map_err(|err| err.unwrap())?;
  }

  let results = voting_system_client.tally();

  assert!(results.get(project001_id).unwrap() == (3, 0));
  assert!(results.get(project002_id).unwrap() == (1, 0));
  assert!(results.get(project003_id).unwrap() == (0, 0));

  Ok(())
}

#[test]
pub fn test_vote_with_different_options() -> Result<(), VotingSystemError> {
  let env = Env::default();
  env.budget().reset_unlimited();

  let voting_system_id = env.register_contract(None, VotingSystem);
  let voting_system_client = VotingSystemClient::new(&env, &voting_system_id);

  let neural_governance_id = env.register_contract_wasm(None, neural_governance_contract::WASM);
  let neural_governance_client =
    neural_governance_contract::Client::new(&env, &neural_governance_id);

  // external data provider
  let external_data_provider_id =
    env.register_contract_wasm(None, external_data_provider_contract::WASM);
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_id);
  external_data_provider_client.mock_sample_data();

  // layer 1
  let layer_1_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_1_client = layer_contract::Client::new(&env, &layer_1_id);
  {
    let simple_neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);

    let assigned_reputation_neuron_id =
      env.register_contract_wasm(None, assigned_reputation_neuron_contract::WASM);
    let assigned_reputation_neuron_client =
      assigned_reputation_neuron_contract::Client::new(&env, &assigned_reputation_neuron_id);
    assigned_reputation_neuron_client.set_external_data_provider(&external_data_provider_id);

    layer_1_client.set_layer_aggregator(&LayerAggregator::SUM);
    layer_1_client.add_neuron(&simple_neuron_id);
    layer_1_client.add_neuron(&assigned_reputation_neuron_id);
  }

  // layer 2
  let layer_2_id = env.register_contract_wasm(None, layer_contract::WASM);
  let layer_2_client = layer_contract::Client::new(&env, &layer_2_id);
  {
    let simple_neuron_id = env.register_contract_wasm(None, simple_neuron_contract::WASM);
    let assigned_reputation_neuron_id =
      env.register_contract_wasm(None, assigned_reputation_neuron_contract::WASM);
    let assigned_reputation_neuron_client =
      assigned_reputation_neuron_contract::Client::new(&env, &assigned_reputation_neuron_id);
    assigned_reputation_neuron_client.set_external_data_provider(&external_data_provider_id);

    let prior_voting_history_neuron_id =
      env.register_contract_wasm(None, prior_voting_history_neuron_contract::WASM);
    let prior_voting_history_neuron_client =
      assigned_reputation_neuron_contract::Client::new(&env, &prior_voting_history_neuron_id);
    prior_voting_history_neuron_client.set_external_data_provider(&external_data_provider_id);

    layer_2_client.set_layer_aggregator(&LayerAggregator::SUM);
    layer_2_client.add_neuron(&simple_neuron_id);
    layer_2_client.add_neuron(&assigned_reputation_neuron_id);
    layer_2_client.add_neuron(&prior_voting_history_neuron_id);
  }

  neural_governance_client.add_layer(&layer_1_id);
  neural_governance_client.add_layer(&layer_2_id);

  voting_system_client.set_neural_governance(&neural_governance_id);
  assert!(voting_system_client.get_projects().is_empty());
  assert!(voting_system_client.tally().is_empty());
  assert!(voting_system_client.get_votes().is_empty());

  let project001_id = String::from_slice(&env, "project001");
  let project002_id = String::from_slice(&env, "project002");
  let project003_id = String::from_slice(&env, "project003");

  let _ = voting_system_client
    .try_add_project(&project001_id)
    .map_err(|err| err.unwrap())?;
  let _ = voting_system_client
    .try_add_project(&project002_id)
    .map_err(|err| err.unwrap())?;
  let _ = voting_system_client
    .try_add_project(&project003_id)
    .map_err(|err| err.unwrap())?;
  let user001_id = String::from_slice(&env, "user001");
  {
    let _ = voting_system_client
      .try_vote(&user001_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user001_id, &project002_id, &Vote::NO)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user001_id, &project003_id, &Vote::ABSTAIN)
      .map_err(|err| err.unwrap())?;
  }
  let user002_id = String::from_slice(&env, "user002");
  {
    let _ = voting_system_client
      .try_vote(&user002_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user002_id, &project002_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user002_id, &project003_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
  }
  let user003_id = String::from_slice(&env, "user003");
  {
    let _ = voting_system_client
      .try_vote(&user003_id, &project001_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user003_id, &project002_id, &Vote::YES)
      .map_err(|err| err.unwrap())?;
    let _ = voting_system_client
      .try_vote(&user003_id, &project003_id, &Vote::NO)
      .map_err(|err| err.unwrap())?;
  }

  let votes = voting_system_client.get_votes();

  let project001_votes = votes.get(project001_id.clone()).unwrap();
  assert!(project001_votes.get(user001_id.clone()).unwrap() == Vote::YES);
  assert!(project001_votes.get(user002_id.clone()).unwrap() == Vote::YES);
  assert!(project001_votes.get(user003_id.clone()).unwrap() == Vote::YES);

  let project002_votes = votes.get(project002_id.clone()).unwrap();
  assert!(project002_votes.get(user001_id.clone()).unwrap() == Vote::NO);
  assert!(project002_votes.get(user002_id.clone()).unwrap() == Vote::YES);
  assert!(project002_votes.get(user003_id.clone()).unwrap() == Vote::YES);

  let project003_votes = votes.get(project003_id.clone()).unwrap();
  assert!(project003_votes.get(user001_id.clone()).unwrap() == Vote::ABSTAIN);
  assert!(project003_votes.get(user002_id.clone()).unwrap() == Vote::YES);
  assert!(project003_votes.get(user003_id.clone()).unwrap() == Vote::NO);

  let results = voting_system_client
    .try_tally()
    .map_err(|err| err.unwrap())?
    .unwrap();

  assert!(results.get(project001_id).unwrap() == (11, 26));
  assert!(results.get(project002_id).unwrap() == (4, 186));
  assert!(results.get(project003_id).unwrap() == (0, 994));

  Ok(())
}
