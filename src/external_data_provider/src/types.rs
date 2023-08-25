use soroban_sdk::contracterror;

pub const MIN_DELEGATEES: u32 = 5;
pub const MAX_DELEGATEES: u32 = 10;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ExternalDataProviderError {
  UnknownError = 0,
  TooManyDelegatees = 1,
  NotEnoughDelegatees = 2,
}
