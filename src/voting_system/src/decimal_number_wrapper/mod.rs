#![allow(dead_code)]
static DECIMAL_POINTS: u32 = 3;
static DECIMAL_MODIFIER: u32 = (10 as u32).pow(DECIMAL_POINTS);

#[derive(Default)]
pub struct DecimalNumberWrapper {
  pub whole: u32,
  pub fractional: u32,
}

impl DecimalNumberWrapper {
  pub fn new(value: &str) -> Self {
    let res = DecimalNumberWrapper::from(value);
    res.validate()
  }

  fn validate(self) -> Self {
    if self.fractional >= DECIMAL_MODIFIER {
      panic!("franctional number exceeded the limit")
    }
    self
  }

  fn prepare_number(number: DecimalNumberWrapper) -> u32 {
    number.whole * DECIMAL_MODIFIER + number.fractional
  }

  pub fn add(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    let result = DecimalNumberWrapper::prepare_number(a.validate())
      + DecimalNumberWrapper::prepare_number(b.validate());
    let whole = result / DECIMAL_MODIFIER;

    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn mul(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    let result = DecimalNumberWrapper::prepare_number(a.validate())
      * DecimalNumberWrapper::prepare_number(b.validate());
    let result = result / DECIMAL_MODIFIER;
    let whole = result / DECIMAL_MODIFIER;
    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn div(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    if b.as_raw() == 0 {
      panic!("division by zero")
    }
    let a_prepared = DecimalNumberWrapper::prepare_number(a.validate()) * DECIMAL_MODIFIER;
    let result = a_prepared / DecimalNumberWrapper::prepare_number(b.validate());
    let whole = result / DECIMAL_MODIFIER;
    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  // a - b
  // we operate on unsigned values, so in case a < b, it just returns 0
  pub fn sub(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    let num_a = DecimalNumberWrapper::prepare_number(a);
    let num_b = DecimalNumberWrapper::prepare_number(b);
    if num_a <= num_b {
      return Default::default();
    }
    let result = num_a - num_b;
    let whole = result / DECIMAL_MODIFIER;

    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  // 0 - equal
  // 1 - a > b
  // 2 - a < b
  pub fn cmp(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> u32 {
    let num_a = DecimalNumberWrapper::prepare_number(a);
    let num_b = DecimalNumberWrapper::prepare_number(b);
    if num_a == num_b {
      return 0;
    }
    if num_a > num_b {
      return 1;
    }
    2
  }

  pub fn as_tuple(&self) -> (u32, u32) {
    (self.whole, self.fractional)
  }

  pub fn as_raw(&self) -> u32 {
    self.whole * DECIMAL_MODIFIER + self.fractional
  }
}

impl From<(u32, u32)> for DecimalNumberWrapper {
  fn from(value: (u32, u32)) -> Self {
    DecimalNumberWrapper {
      whole: value.0,
      fractional: value.1,
    }
    .validate()
  }
}

impl From<&str> for DecimalNumberWrapper {
  fn from(value: &str) -> Self {
    let mut split = value.split(".");
    if split.clone().count() as u32 != 2 {
      panic!(
        "invalid decimal point number, it should be delimited by a single ., for example 12.34"
      );
    }
    let whole = split.nth(0).unwrap().trim().parse().unwrap();
    let fractional_str = split.nth(0).unwrap();
    let mut fractional: u32 = fractional_str.trim().parse().unwrap();

    if fractional != 0 {
      let mut prefixing_zeros = 0;
      let mut chars = fractional_str.clone().chars();
      loop {
        let c = chars.nth(0).unwrap_or('?');
        if c != '0' {
          break;
        }
        prefixing_zeros += 1;
      }

      let treshold = (10 as u32).pow(2 - prefixing_zeros);
      while fractional < treshold {
        fractional = fractional * 10;
      }
    }
    DecimalNumberWrapper { whole, fractional }.validate()
  }
}

impl From<u32> for DecimalNumberWrapper {
  fn from(raw: u32) -> Self {
    let whole = raw / DECIMAL_MODIFIER;

    DecimalNumberWrapper {
      whole,
      fractional: raw - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }
}

#[cfg(test)]
mod decimal_number_wrapper_test;
