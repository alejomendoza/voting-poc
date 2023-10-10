use crate::types::VotingSystemError;
use soroban_sdk::{Env, String};

pub fn oracle_function(
  _env: Env,
  _voter_id: String,
  _submission_id: String,
) -> Result<(u32, u32), VotingSystemError> {
  Ok((1, 100))
}
