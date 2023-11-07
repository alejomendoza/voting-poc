use crate::{external_data_provider_contract, types::VotingSystemError, VotingSystem};
use soroban_decimal_numbers::DecimalNumberWrapper;
use soroban_sdk::{Env, String};

pub fn oracle_function(
  env: Env,
  voter_id: String,
  external_data_provider_client: &external_data_provider_contract::Client,
) -> Result<(u32, u32), VotingSystemError> {
  let voter_active_rounds = external_data_provider_client.get_user_prior_voting_history(&voter_id);
  if voter_active_rounds.is_empty() {
    return Ok((0, 0));
  }
  let round_bonus_map = external_data_provider_client.get_round_bonus_map();
  let mut bonus_result = DecimalNumberWrapper::new("0.0");
  for round in voter_active_rounds {
    let bonus: (u32, u32) = round_bonus_map.get(round).unwrap_or((0, 0));
    bonus_result = DecimalNumberWrapper::add(
      DecimalNumberWrapper::from(bonus_result),
      DecimalNumberWrapper::from(bonus),
    );
  }
  Ok(bonus_result.as_tuple())
}
