use crate::types::VotingSystemError;
use soroban_sdk::{Env, String};

pub fn oracle_function(
  _env: Env,
  _voter_id: String,
  _project_id: String,
  _maybe_previous_layer_vote: Option<(u32, u32)>,
) -> Result<(u32, u32), VotingSystemError> {
  unimplemented!();
}
