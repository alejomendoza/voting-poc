use soroban_sdk::{contracterror, contracttype, Env, String};

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
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayerAggregator {
  Unknown,
  Sum,
  Product,
}

pub fn layer_aggregator_from_str(env: Env, str: String) -> LayerAggregator {
  if str == String::from_slice(&env, "Sum") {
    return LayerAggregator::Sum;
  }
  if str == String::from_slice(&env, "Product") {
    return LayerAggregator::Product;
  }
  LayerAggregator::Unknown
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NeuronType {
  Dummy,
  AssignedReputation,
  PriorVotingHistory,
}

pub fn neuron_type_from_str(env: Env, str: String) -> Result<NeuronType, VotingSystemError> {
  if str == String::from_slice(&env, "Dummy") {
    return Ok(NeuronType::Dummy);
  }
  if str == String::from_slice(&env, "AssignedReputation") {
    return Ok(NeuronType::AssignedReputation);
  }
  if str == String::from_slice(&env, "PriorVotingHistory") {
    return Ok(NeuronType::PriorVotingHistory);
  }
  Err(VotingSystemError::UnknownNeuronType)
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
  UnknownNeuronType = 20,
}
