use soroban_sdk::{contracterror, contracttype, Env, String};

pub type DecimalNumber = (u32, u32);

pub type UserUUID = String;
pub type ProjectUUID = String;

pub static DEFAULT_WEIGHT: DecimalNumber = (1, 0);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayerAggregator {
  UNKNOWN,
  SUM,
  PRODUCT,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Vote {
  YES = 2,
  NO = 1,
  ABSTAIN = 0,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReputationCategory {
  Excellent = 5,
  VeryGood = 4,
  Good = 3,
  Average = 2,
  Poor = 1,
  Uncategorized = 0,
}

// todo maybe move this outside of "types"
pub fn get_reputation_category_bonus(reputation_category: ReputationCategory) -> u32 {
  match reputation_category {
    ReputationCategory::Uncategorized | ReputationCategory::Poor => 0,
    other => other as u32,
  }
}

pub enum RoundAction {
  VOTE,
  DELEGATE,
  ABSTAIN,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VotingSystemError {
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
}

pub trait Neuron {
  fn oracle_function(
    env: Env,
    voter_id: UserUUID,
    project_id: ProjectUUID,
    maybe_previous_layer_vote: Option<DecimalNumber>,
  ) -> Result<DecimalNumber, VotingSystemError>;
  fn weight_function(env: Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber;
  fn set_weight(env: Env, new_weight: DecimalNumber);
}
