use crate::{external_data_provider_contract, types::VotingSystemError};
use soroban_sdk::{Env, String};

pub fn oracle_function(
  _env: Env,
  voter_id: String,
  external_data_provider_client: &external_data_provider_contract::Client,
) -> Result<(u32, u32), VotingSystemError> {
  let rank = external_data_provider_client.get_page_rank_result_for_user(&voter_id);

  Ok(rank)
}
