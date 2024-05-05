use std::cmp::PartialEq;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::AddAssign;

fn round(value: f32, digits: i32) -> f32 {
  let multiplier = 10_f32.powi(digits);
  (value * multiplier).round() / multiplier
}

/// A unit of weight
/// used as the basis for ingridient measurements
///
/// # Fields
///
/// * `value`: The value of the gram, in grams.
///
/// # Example
///
///
/// let flour_weight = Gram(500.0);
/// assert_eq!(flour_weight.0, 500.0);
///
/// let water_weight = Gram::From(12.0);
/// assert_eq!(water_weight.0, 12.0);
///
/// let starter_weight: Gram = 10.into();
/// assert_eq!(starter_weight.0, 10.0);
#[derive(Debug, Clone, Copy)]
pub struct Gram(pub f32);

impl Gram {
  pub const ZERO: Self = Self(0.0);

  pub fn as_ratio_of<T>(self, other: &Self) -> T
    where
      T: From<f32>,
    {
      // Percent::From(f32) expects a percentage like 10 for 10%, 15.2 for 15.2%
      // while a division of floats yields a decimal fraction like 0.1 and 0.152
      // Hence the multiplication by 100
      // The T::from is responsible to round up by the DECIMAL resolution.
      T::from(self.0 / other.0 * 100.0 )
    }
}

impl From<i32> for Gram {
  fn from(value: i32) -> Self {
    Gram(value as f32)
  }
}

impl From<f32> for Gram {
  fn from(value: f32) -> Self {
    Gram(value)
  }
}

// Operator overloading
impl AddAssign<Gram> for Gram {
  fn add_assign(& mut self, other: Self) {
    self.0 += other.0;
  }
}

impl Add<Gram> for Gram {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Gram(self.0 + other.0)
  }
}

impl Add<i32> for Gram {
  type Output = Self;
  fn add(self, other: i32) -> Self {
    Gram(self.0 + other as f32)
  }
}

impl Add<f32> for Gram {
  type Output = Self;
  fn add(self, other: f32) -> Self {
    Gram(self.0 + other)
  }
}

impl Sub<Gram> for Gram {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    self - other.0
  }
}

impl Sub<i32> for Gram {
  type Output = Self;
  fn sub(self, other: i32) -> Self {
    self - other as f32
  }
}

impl Sub<f32> for Gram {
  type Output = Self;
  fn sub(self, other: f32) -> Self {
    assert!(
      (self.0 - other) > 0.0,
      "negative weight not allowed. Subtration {} - {} = {} panics",
      self.0,
      other,
      self.0 - other
    );

    Gram(self.0 - other)
  }
}

impl Mul<i32> for Gram {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    Gram(round(self.0 * other as f32, 2))
  }
}

impl Mul<f32> for Gram {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    Gram(round(self.0 * other, 2))
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Mul<Percent<MIN, MAX, DECIMALS>> for Gram {
  type Output = Self;
  fn mul(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    // let normalizer: f32 = Percent::DECIMALS_MULTIPLIER as f32 * 100.0f32;
    let normalizer: f32 = Percent::<MIN, MAX, DECIMALS>::DECIMALS_MULTIPLIER as f32 * 100.0f32;
    Gram(round(self.0 * (other.0 as f32 / normalizer), 2))
  }
}

impl Div<i32> for Gram {
  type Output = Self;
  fn div(self, other: i32) -> Self {
    Gram(round(self.0 / other as f32, 2))
  }
}

impl Div<f32> for Gram {
  type Output = Self;
  fn div(self, other: f32) -> Self {
    Gram(round(self.0 / other, 2))
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Div<Percent<MIN, MAX, DECIMALS>> for Gram {
  type Output = Self;
  fn div(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    let normalizer: f32 = Percent::<MIN, MAX, DECIMALS>::DECIMALS_MULTIPLIER as f32 * 100.0f32;
    Gram(round(self.0 / (other.0 as f32 / normalizer), 2))
  }
}

impl PartialEq for Gram {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl fmt::Display for Gram {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:.2} g", self.0)
  }
}

// A limit bounded percentage abstraction.
// Fractional percentages are not supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Percent<const MIN: usize, const MAX: usize, const DECIMALS: usize>(pub usize);

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Percent<MIN, MAX, DECIMALS> {
  pub const ZERO: Self = Self(0);
  pub const MIN: Self = Self(MIN);
  pub const MAX: Self = Self(MAX);
  pub const DECIMALS: usize = DECIMALS;
  pub const DECIMALS_MULTIPLIER: usize = 10usize.pow(DECIMALS as u32);

  pub fn new(value: usize) -> Self {
    assert!(
      MIN * Self::DECIMALS_MULTIPLIER <= value && value <= MAX * Self::DECIMALS_MULTIPLIER,
      "value {} must be between {} and {} including",
      value,
      MIN,
      MAX
    );
    Self(value)
  }

  pub fn valid_new(value: usize) -> Option<Self> {
    if (MIN * Self::DECIMALS_MULTIPLIER) <= value && value <= (MAX * Self::DECIMALS_MULTIPLIER) {
      return Some(Self(value));
    } 
    None
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<usize> for Percent<MIN, MAX, DECIMALS> {
  fn from(value: usize) -> Self {
    Percent::new(value * Self::DECIMALS_MULTIPLIER)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<i32> for Percent<MIN, MAX, DECIMALS> {
  fn from(value: i32) -> Self {
    Percent::new(value as usize * Self::DECIMALS_MULTIPLIER)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS : usize> From<f32> for Percent<MIN, MAX, DECIMALS> {
  fn from(value: f32) -> Self {
    let rounding_decimal = 0.5 / 10_f32.powf(DECIMALS as f32);
    Percent::new(((value + rounding_decimal) * Self::DECIMALS_MULTIPLIER as f32) as usize)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> fmt::Display for Percent<MIN, MAX, DECIMALS> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:.DECIMALS$}%", self.0 as f32 / Self::DECIMALS_MULTIPLIER as f32)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Div<Percent<MIN, MAX, DECIMALS>> for f32 {
  type Output = Self;
  fn div(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    round(self / (other.0 as f32 / 100.0), 2)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Add<Percent<MIN, MAX, DECIMALS>> for Percent<MIN, MAX, DECIMALS> {
  type Output = Self;
  fn add(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    Self(self.0 + other.0)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Sum for Percent<MIN, MAX, DECIMALS> {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Self>,
  {
    iter.fold(Self(0), |acc, x| acc + x)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum  Measure<const MIN: usize, const MAX: usize, const DECIMALS: usize>  {
  Weight(Gram),
  Ratio(Percent<MIN, MAX, DECIMALS>),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gram_constructor() {
    let weight = Gram(500.0);
    assert_eq!(weight.0, 500.0);
  }

  #[test]
  fn test_gram_from_int() {
    let weight = Gram::from(312);
    assert_eq!(weight.0, 312.0);
  }

  #[test]
  fn test_gram_from_float() {
    let weight = Gram::from(312.5);
    assert_eq!(weight.0, 312.5);
  }

  #[test]
  fn test_gram_into_int() {
    let weight: Gram = 11.into();
    assert_eq!(weight.0, 11.0);
  }

  #[test]
  fn test_gram_into_float() {
    let weight: Gram = 11.3.into();
    assert_eq!(weight.0, 11.3);
  }

  #[test]
  fn test_gram_add() {
    let weight1: Gram = 11.3.into();
    let weight2: Gram = 11.into();
    assert_eq!(weight1 + weight2, 22.3.into());
  }

  #[test]
  fn test_gram_add_i32() {
    let weight1: Gram = 11.3.into();
    assert_eq!(weight1 + 3, 14.3.into());
  }

  #[test]
  fn test_gram_add_f32() {
    let weight1: Gram = 11.3.into();
    assert_eq!(weight1 + 3.5, 14.8.into());
  }

  #[test]
  fn test_gram_mul_i32() {
    let weight1: Gram = 11.3.into();
    assert_eq!(weight1 * 3, 33.9.into());
  }

  #[test]
  fn test_gram_mul_f32() {
    let weight1: Gram = 11.4.into();
    assert_eq!(weight1 * 3.6, 41.04.into());
  }

  #[test]
  fn test_gram_div_i32() {
    let weight1: Gram = 33.9.into();
    assert_eq!(weight1 / 3, 11.3.into());
  }

  #[test]
  fn test_gram_div_f32() {
    let weight1: Gram = 41.04.into();
    assert_eq!(weight1 / 3.6, 11.4.into());
  }
}
