static DECIMAL_POINTS: u32 = 3;
static DECIMAL_MODIFIER: u32 = (10 as u32).pow(DECIMAL_POINTS);

pub struct DecimalNumberPersist {
  pub whole: u32,
  pub fractional: u32,
}

impl DecimalNumberPersist {
  pub fn new(whole: u32, fractional: u32) -> Self {
    let res = DecimalNumberPersist { whole, fractional };
    res.validate()
  }

  fn validate(self) -> Self {
    if self.fractional >= DECIMAL_MODIFIER {
      panic!("franctional number exceeded the limit")
    }
    self
  }

  fn prepare_number(number: DecimalNumberPersist) -> u32 {
    number.whole * DECIMAL_MODIFIER + number.fractional
  }

  pub fn add(a: DecimalNumberPersist, b: DecimalNumberPersist) -> DecimalNumberPersist {
    let result = DecimalNumberPersist::prepare_number(a.validate())
      + DecimalNumberPersist::prepare_number(b.validate());
    let whole = result / DECIMAL_MODIFIER;

    DecimalNumberPersist {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn mul(a: DecimalNumberPersist, b: DecimalNumberPersist) -> DecimalNumberPersist {
    let result = DecimalNumberPersist::prepare_number(a.validate())
      * DecimalNumberPersist::prepare_number(b.validate());
    let result = result / DECIMAL_MODIFIER;
    let whole = result / DECIMAL_MODIFIER;
    DecimalNumberPersist {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn as_tuple(&self) -> (u32, u32) {
    (self.whole, self.fractional)
  }
}

impl From<(u32, u32)> for DecimalNumberPersist {
  fn from(value: (u32, u32)) -> Self {
    DecimalNumberPersist {
      whole: value.0,
      fractional: value.1,
    }
  }
}

#[cfg(test)]
mod test;
