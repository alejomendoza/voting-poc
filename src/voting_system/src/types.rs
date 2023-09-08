use soroban_sdk::{contracterror, contracttype};

pub type DecimalNumber = (u32, u32);

pub static DEFAULT_WEIGHT: DecimalNumber = (1, 0);

pub const QUORUM_SIZE: u32 = 5;
pub const QUORUM_PARTICIPATION_TRESHOLD: u32 = 3;

pub const ABSTAIN_VOTING_POWER: (u32, u32) = (0, 0);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Vote {
  Yes,
  No,
  Abstain,
  Delegate,
}

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
}
