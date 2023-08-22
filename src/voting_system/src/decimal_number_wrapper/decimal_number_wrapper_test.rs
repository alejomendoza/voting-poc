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

}

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

  assert!(
    DecimalNumberWrapper::mul(
      DecimalNumberWrapper::new(4, 400),
      DecimalNumberWrapper::new(0, 200)
    )
    .as_tuple()
      == (0, 880)
  );
}

#[test]
pub fn test_cmp() {
  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new(14, 450),
      DecimalNumberWrapper::new(14, 450)
    ) == 0
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new(15, 450),
      DecimalNumberWrapper::new(14, 450)
    ) == 1
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new(14, 450),
      DecimalNumberWrapper::new(114, 450)
    ) == 2
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new(14, 460),
      DecimalNumberWrapper::new(14, 450)
    ) == 1
  );

  assert!(
    DecimalNumberWrapper::cmp(
      DecimalNumberWrapper::new(14, 450),
      DecimalNumberWrapper::new(14, 460)
    ) == 2
  );
}

#[test]
pub fn test_sub() {
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new(12, 360),
      DecimalNumberWrapper::new(12, 340)
    )
    .as_tuple()
      == (0, 20)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new(12, 360),
      DecimalNumberWrapper::new(12, 361)
    )
    .as_tuple()
      == (0, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new(13, 360),
      DecimalNumberWrapper::new(12, 360)
    )
    .as_tuple()
      == (1, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new(13, 360),
      DecimalNumberWrapper::new(14, 360)
    )
    .as_tuple()
      == (0, 0)
  );
  assert!(
    DecimalNumberWrapper::sub(
      DecimalNumberWrapper::new(14, 360),
      DecimalNumberWrapper::new(14, 360)
    )
    .as_tuple()
      == (0, 0)
  );
}
