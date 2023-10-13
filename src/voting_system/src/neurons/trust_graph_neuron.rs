use crate::{
  external_data_provider_contract, page_rank::Rank, types::VotingSystemError, VotingSystem,
};
use soroban_sdk::{Env, String};

pub fn oracle_function(
  env: Env,
  voter_id: String,
  _submission_id: String,
) -> Result<(u32, u32), VotingSystemError> {
  let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_address);

  let rank = Rank::from_pages(&env, external_data_provider_client.get_trust_map());
  // // TODO: consider caching the result in external data provider so the algorithm's not run every time this neuron is executed
  let calculated = rank.calculate(&env); // <--- if you uncomment this, it will hang forever when called from JS
  // let rank = calculated.get(voter_id.clone()).unwrap_or((0, 0));

  // Ok(rank)
  Ok((1, 0))
}
