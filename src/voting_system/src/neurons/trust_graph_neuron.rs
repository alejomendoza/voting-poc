use crate::{external_data_provider_contract, types::VotingSystemError, VotingSystem};
use soroban_sdk::{Env, String};

pub fn oracle_function(env: Env, voter_id: String) -> Result<(u32, u32), VotingSystemError> {
  let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider_address);

  let rank = external_data_provider_client.get_page_rank_result_for_user(&voter_id);

  Ok(rank)
}
