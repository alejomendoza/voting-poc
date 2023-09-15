use crate::{external_data_provider_contract, types::VotingSystemError, VotingSystem};
use soroban_sdk::{Env, String, Address};
use soroban_sdk::FromVal;

pub fn oracle_function(
  env: Env,
  voter_id: String,
  _project_id: String,
) -> Result<(u32, u32), VotingSystemError> {
  let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &Address::from_val(&env, external_data_provider_address.as_val()));

  let reputation_category = external_data_provider_client.get_user_reputation_category(&voter_id);
  let bonus = external_data_provider_client.get_reputation_score(&reputation_category);

  Ok(bonus)
}
