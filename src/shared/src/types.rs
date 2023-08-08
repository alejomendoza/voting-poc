use soroban_sdk::{contracttype, Env, String};

pub type DecimalNumber = (u32, u32);

pub type UserUUID = String;
pub type ProjectUUID = String;

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

pub trait Neuron {
  fn oracle_function(
    env: Env,
    voter_id: UserUUID,
    project_id: ProjectUUID,
    maybe_previous_layer_vote: Option<DecimalNumber>,
  ) -> DecimalNumber;
  fn weight_function(env: Env, raw_neuron_vote: DecimalNumber) -> DecimalNumber;
}
