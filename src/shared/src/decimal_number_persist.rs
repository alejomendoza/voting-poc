static DECIMAL_MODIFIER: u32 = 1000;

pub struct DecimalNumberPersist {
  pub whole: u32,
  pub fractional: u32,
}

impl DecimalNumberPersist {
  pub fn to_float(&self) -> f32 {
    let mut result: f32 = self.whole as f32;
    result += (self.fractional as f32) / (DECIMAL_MODIFIER as f32);
    result
  }

  pub fn as_tuple(&self) -> (u32, u32) {
    (self.whole, self.fractional)
  }

  pub fn add(a: DecimalNumberPersist, b: DecimalNumberPersist) -> DecimalNumberPersist {
    let n1 = a.whole * DECIMAL_MODIFIER + a.fractional;
    let n2 = b.whole * DECIMAL_MODIFIER + b.fractional;
    let result = n1 + n2;
    let whole = result / DECIMAL_MODIFIER;
    DecimalNumberPersist {
      whole: whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
  }

  pub fn mul(a: DecimalNumberPersist, b: DecimalNumberPersist) -> DecimalNumberPersist {
    let n1 = a.whole * DECIMAL_MODIFIER + a.fractional;
    let n2 = b.whole * DECIMAL_MODIFIER + b.fractional;
    let result = n1 * n2;
    let whole = result / (DECIMAL_MODIFIER * DECIMAL_MODIFIER);
    DecimalNumberPersist {
      whole,
      fractional: result - (whole * (DECIMAL_MODIFIER * DECIMAL_MODIFIER)),
    }
  }
}

impl From<f32> for DecimalNumberPersist {
  fn from(value: f32) -> Self {
    let whole = value as u32;
    let fractional =
      ((value * (DECIMAL_MODIFIER as f32)) - ((whole as f32) * (DECIMAL_MODIFIER as f32))) as u32;
    DecimalNumberPersist { whole, fractional }
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
