use soroban_sdk::{Env, String};
use voting_shared::{types::{VotingSystemError, DecimalNumber}, decimal_number_wrapper::DecimalNumberWrapper};

pub fn oracle_function(
  _env: Env,
  _voter_id: String,
  _project_id: String,
  maybe_previous_layer_vote: Option<(u32, u32)>,
) -> Result<(u32, u32), VotingSystemError> {
  if let Some(previous_layer_vote) = maybe_previous_layer_vote {
    return Ok((previous_layer_vote.0 + 1, previous_layer_vote.1 + 100));
  }
  Ok((1, 0))
}
