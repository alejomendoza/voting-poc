use soroban_sdk::{contracterror, contracttype, Env, String};

pub type DecimalNumber = (u32, u32);

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ExternalDataProviderError {
  UnknownError = 0,
  TooManyDelegatees = 1,
  NotEnoughDelegatees = 2,
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

pub fn reputation_category_from_str(env: &Env, str: String) -> ReputationCategory {
  if str == String::from_slice(&env, "Poor") {
    return ReputationCategory::Poor;
  }
  if str == String::from_slice(&env, "Average") {
    return ReputationCategory::Average;
  }
  if str == String::from_slice(&env, "Good") {
    return ReputationCategory::Good;
  }
  if str == String::from_slice(&env, "VeryGood") {
    return ReputationCategory::VeryGood;
  }
  if str == String::from_slice(&env, "Excellent") {
    return ReputationCategory::Excellent;
  }
  return ReputationCategory::Uncategorized;
}
