use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ExternalDataProviderError {
  UnknownError = 0,
  TooManyDelegatees = 1,
  NotEnoughDelegatees = 2,
}
