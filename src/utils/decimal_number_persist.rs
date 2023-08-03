static DECIMAL_MODIFIER: u32 = 1000;

pub struct DecimalNumberPersist {
  whole: u32,
  fractional: u32,
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
}

impl From<f32> for DecimalNumberPersist {
  fn from(value: f32) -> Self {
    let whole = value as u32;
    let fractional = ((value * (DECIMAL_MODIFIER as f32)) as u32) - (whole * DECIMAL_MODIFIER);
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
