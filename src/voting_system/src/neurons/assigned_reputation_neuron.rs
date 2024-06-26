use crate::{external_data_provider_contract, types::VotingSystemError};
use soroban_sdk::{Env, String};

pub fn oracle_function(
  _env: Env,
  voter_id: String,
  external_data_provider_client: &external_data_provider_contract::Client,
) -> Result<(u32, u32), VotingSystemError> {
  let reputation_category = external_data_provider_client.get_user_reputation_category(&voter_id);
  let bonus = external_data_provider_client.get_reputation_score(&reputation_category);

  Ok(bonus)
}
