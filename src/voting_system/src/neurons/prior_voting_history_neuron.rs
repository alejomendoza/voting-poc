use crate::{
  decimal_number_wrapper::DecimalNumberWrapper, external_data_provider_contract,
  types::VotingSystemError, VotingSystem,
};
use soroban_sdk::{Env, String};

pub fn oracle_function(
  env: Env,
  voter_id: String,
  _project_id: String,
  maybe_previous_layer_vote: Option<(u32, u32)>,
) -> Result<(u32, u32), VotingSystemError> {
  let external_data_provider = VotingSystem::get_external_data_provider(env.clone())?;
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &external_data_provider);

  let voter_active_rounds = external_data_provider_client.get_user_prior_voting_history(&voter_id);
  if voter_active_rounds.is_empty() {
    return Ok((0, 0));
  }
  let round_bonus_map = external_data_provider_client.get_round_bonus_map();
  let previous_layer_vote = maybe_previous_layer_vote.unwrap_or((1, 0));
  let previous_layer_vote: DecimalNumberWrapper = DecimalNumberWrapper::from(previous_layer_vote);
  let mut bonus_result = DecimalNumberWrapper::from(previous_layer_vote.as_tuple());
  for round in voter_active_rounds {
    let bonus: (u32, u32) = round_bonus_map
      .get(round)
      .ok_or(VotingSystemError::RoundNotFoundInRoundBonusMap)?;
    bonus_result = DecimalNumberWrapper::add(
      DecimalNumberWrapper::from(bonus_result),
      DecimalNumberWrapper::from(bonus),
    );
  }
  bonus_result = DecimalNumberWrapper::add(previous_layer_vote, bonus_result);
  Ok(bonus_result.as_tuple())
}