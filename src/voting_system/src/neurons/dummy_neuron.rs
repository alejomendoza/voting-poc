use crate::types::VotingSystemError;
use soroban_sdk::{Env, String};

pub fn oracle_function(
  _env: Env,
  _voter_id: String,
  _project_id: String,
  previous_layer_vote: (u32, u32),
) -> Result<(u32, u32), VotingSystemError> {
  Ok((previous_layer_vote.0 + 1, previous_layer_vote.1 + 100))
}
