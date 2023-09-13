use soroban_sdk::{contracterror, contracttype, String, Env};

pub type DecimalNumber = (u32, u32);

pub static DEFAULT_WEIGHT: DecimalNumber = (1, 0);

pub const QUORUM_SIZE: u32 = 5;
pub const QUORUM_PARTICIPATION_TRESHOLD: u32 = 3;

pub const MIN_DELEGATEES: u32 = 5;
pub const MAX_DELEGATEES: u32 = 10;

pub const INITIAL_VOTING_POWER: (u32, u32) = (0, 0);
pub const ABSTAIN_VOTING_POWER: (u32, u32) = (0, 0);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Vote {
  Yes,
  No,
  Abstain,
  Delegate,
}

pub fn vote_from_str(env: Env, str: String) -> Vote {
  if str == String::from_slice(&env, "Yes") {
    return Vote::Yes;
  }
  if str == String::from_slice(&env, "No") {
    return Vote::No;
  }
  if str == String::from_slice(&env, "Delegate") {
    return Vote::Delegate;
  }
  return Vote::Abstain;
  // match str {
  //   String::from_slice(&env, "Yes") => Vote::Yes,
  //   "No" => Vote::No,
  //   "Abstain" => Vote::Abstain,
  //   "Delegate" => Vote::Delegate,
  //   _ => Vote::Abstain,
  // }
}

// impl FromStr for Vote {
//   type Err = ();

//   fn from_str(input: &str) -> Result<Vote, Self::Err> {
//       match input {
//           "Yes"  => Ok(Vote::Yes),
//           "No"  => Ok(Vote::No),
//           "Abstain"  => Ok(Vote::Abstain),
//           "Delegate" => Ok(Vote::Delegate),
//           _      => Err(()),
//       }
//   }
// }

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VotingSystemError {
  // todo remove unused errors
  UnknownError = 0,
  ExternalDataProviderNotSet = 1,
  LayerAggregatorNotSet = 2,
  NoNeuronsExist = 3,
  CannotRunUnknownLayerAggregator = 4,
  NoLayersExist = 5,
  ProjectDoesNotExist = 6,
  UserAlreadyVoted = 7,
  ProjectAlreadyAdded = 8,
  ReducingvotesForSumAggregatorFailed = 9,
  ReducingvotesForProductAggregatorFailed = 10,
  ResultExpected = 11,
  NeuralGovernanceNotSet = 12,
  RoundNotFoundInRoundBonusMap = 13,
  NoSuchLayer = 14,
  DelegateesNotFound = 15,
  VoteNotFoundForDelegatee = 16,
  UnexpectedValue = 17,
  TooManyDelegatees = 18,
  NotEnoughDelegatees = 19,
}
