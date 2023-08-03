use soroban_sdk::String;

pub type DecimalNumber = (u32, u32);

pub type UserUUID = String;
pub type ProjectUUID = String;

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
