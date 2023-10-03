use crate::decimal_number_wrapper::DecimalNumberWrapper;

#[test]
pub fn test_from_to() {
  let num: u32 = 123456;
  let dnw: DecimalNumberWrapper = num.into();
  assert!(dnw.as_tuple() == (123, 456));
  assert!(dnw.as_raw() == num);

  let dnw: DecimalNumberWrapper = (12, 74).into();
  assert!(dnw.as_tuple() == (12, 74));
  assert!(dnw.as_raw() == 12074);

  let dnw: DecimalNumberWrapper = (12, 100).into();
  assert!(dnw.as_tuple() == (12, 100));
  assert!(dnw.as_raw() == 12100);

  let dnw: DecimalNumberWrapper = "123.56".into();

  assert!(dnw.as_tuple() == (123, 560));
  assert!(dnw.as_raw() == 123560);

  let dnw: DecimalNumberWrapper = "44.1".into();
  assert!(dnw.as_tuple() == (44, 100));
  assert!(dnw.as_raw() == 44100);
}

#[test]
pub fn test_add() {
  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new("4.8"),
      DecimalNumberWrapper::new("5.5")
    )
    .as_tuple()
      == (10, 300)
  );

  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new("0.001"),
      DecimalNumberWrapper::new("0.1")
    )
    .as_tuple()
      == (0, 101)
  );

  assert!(
    DecimalNumberWrapper::add(
      DecimalNumberWrapper::new("0.001"),
      DecimalNumberWrapper::new("0.020"),
    )
    .as_tuple()
      == (0, 21)
  );
}

#[test]
pub fn test_multiply() {
  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new("5.0"),
      DecimalNumberWrapper::new("7.0"),
    )
    .as_tuple()
      == (35, 0)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new("1.5"),
      DecimalNumberWrapper::new("2.8"),
    )
    .as_tuple()
      == (4, 200)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new("1.5"),
      DecimalNumberWrapper::new("2.8"),
    )
    .as_tuple()
      == (4, 200)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new("1.32"),
      DecimalNumberWrapper::new("0.03"),
    )
    .as_tuple()
      == (0, 39)
  );

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new("4.4"),
      DecimalNumberWrapper::new("0.2"),
    )
    .as_tuple()
      == (0, 880)
  );
}

#[test]
pub fn test_cmp() {
  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new("14.45"),
      DecimalNumberWrapper::new("14.45"),
    ) == 0
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new("15.45"),
      DecimalNumberWrapper::new("14.45"),
    ) == 1
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new("14.45"),
      DecimalNumberWrapper::new("114.45"),
    ) == 2
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new("14.46"),
      DecimalNumberWrapper::new("14.45"),
    ) == 1
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new("14.45"),
      DecimalNumberWrapper::new("14.46"),
    ) == 2
  );
}

#[test]
pub fn test_sub() {
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new("12.36"),
      DecimalNumberWrapper::new("12.34"),
    )
    .as_tuple()
      == (0, 20)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new("12.36"),
      DecimalNumberWrapper::new("12.361"),
    )
    .as_tuple()
      == (0, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new("13.36"),
      DecimalNumberWrapper::new("12.36"),
    )
    .as_tuple()
      == (1, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new("13.36"),
      DecimalNumberWrapper::new("14.36"),
    )
    .as_tuple()
      == (0, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new("14.36"),
      DecimalNumberWrapper::new("14.36"),
    )
    .as_tuple()
      == (0, 0)
  );
}

#[test]
pub fn test_divide() {
  assert!(
    DecimalNumberWrapper::div(
      DecimalNumberWrapper::new("6.0"),
      DecimalNumberWrapper::new("3.0"),
    )
    .as_tuple()
      == (2, 0)
  );

  assert!(
    DecimalNumberWrapper::div(
      DecimalNumberWrapper::new("5.0"),
      DecimalNumberWrapper::new("2.0"),
    )
    .as_tuple()
      == (2, 500)
  );

  assert!(
    DecimalNumberWrapper::div(
      DecimalNumberWrapper::new("1.345"),
      DecimalNumberWrapper::new("2.132"),
    )
    .as_tuple()
      == (0, 630)
  );
}
