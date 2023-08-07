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

pub enum Vote {
  YES = 1,
  NO = -1,
  ABSTAIN = 0,
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
