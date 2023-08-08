use crate::decimal_number_persist::DecimalNumberWrapper;

#[test]
pub fn test_add() {
  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new(4, 800),
      DecimalNumberWrapper::new(5, 500)
    )
    .as_tuple()
      == (10, 300)
  );

  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new(0, 1),
      DecimalNumberWrapper::new(0, 100)
    )
    .as_tuple()
      == (0, 101)
  );

  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new(0, 1),
      DecimalNumberWrapper::new(0, 20)
    )
    .as_tuple()
      == (0, 21)
  );
}

#[test]
pub fn test_multiply() {
  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new(5, 0),
      DecimalNumberWrapper::new(7, 0)
    )
    .as_tuple()
      == (35, 0)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new(1, 500),
      DecimalNumberWrapper::new(2, 800)
    )
    .as_tuple()
      == (4, 200)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new(1, 5),
      DecimalNumberWrapper::new(2, 8)
    )
    .as_tuple()
      == (2, 18)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new(1, 320),
      DecimalNumberWrapper::new(0, 30)
    )
    .as_tuple()
      == (0, 39)
  );
}
