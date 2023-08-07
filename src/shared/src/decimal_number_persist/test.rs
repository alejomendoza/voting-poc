use soroban_sdk::{log, testutils::Logs, Env};

use crate::decimal_number_persist::DecimalNumberPersist;

#[test]
pub fn test_add() {
  let env = Env::default();

  assert!(
    DecimalNumberPersist::add(
      DecimalNumberPersist::new(4, 800),
      DecimalNumberPersist::new(5, 500)
    )
    .as_tuple()
      == (10, 300)
  );

  assert!(
    DecimalNumberPersist::add(
      DecimalNumberPersist::new(0, 1),
      DecimalNumberPersist::new(0, 100)
    )
    .as_tuple()
      == (0, 101)
  );

  assert!(
    DecimalNumberPersist::add(
      DecimalNumberPersist::new(0, 1),
      DecimalNumberPersist::new(0, 20)
    )
    .as_tuple()
      == (0, 21)
  );
}

#[test]
pub fn test_multiply() {
  assert!(
    DecimalNumberPersist::mul(
      DecimalNumberPersist::new(5, 0),
      DecimalNumberPersist::new(7, 0)
    )
    .as_tuple()
      == (35, 0)
  );

  assert!(
    DecimalNumberPersist::mul(
      DecimalNumberPersist::new(1, 500),
      DecimalNumberPersist::new(2, 800)
    )
    .as_tuple()
      == (4, 200)
  );

  assert!(
    DecimalNumberPersist::mul(
      DecimalNumberPersist::new(1, 5),
      DecimalNumberPersist::new(2, 8)
    )
    .as_tuple()
      == (2, 18)
  );

  assert!(
    DecimalNumberPersist::mul(
      DecimalNumberPersist::new(1, 320),
      DecimalNumberPersist::new(0, 30)
    )
    .as_tuple()
      == (0, 39)
  );
}
