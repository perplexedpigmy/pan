use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::common::percent::*;
use num_traits::Num;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

/// Result of dividing weight by weight yields a percent ratio
pub type Ratio = Percent<0, 1_000_0000, 0>;

/// A unit of mass
/// used as the basis for ingridient measurements
///
/// # Fields
///
/// * `value`: The value of the gram, in grams.
///
/// Note:
/// a. A Gram will never be negative
/// b. Constructing it from a value that can't be represented as a Decimal, resolves, implicitly to ZERO
///
/// # Example
///
/// let flour_weight = Gram(500.0);
/// assert_eq!(flour_weight.0, 500.0);
///
/// let water_weight = Gram::From(12.0);
/// assert_eq!(water_weight.0, 12.0);
///
/// let starter_weight: Gram = 10.into();
/// assert_eq!(starter_weight.0, 10.0);
#[derive(Debug, PartialOrd, Clone, Eq, Ord, Copy)]
pub struct Gram(pub Decimal);

impl Gram {
  pub const ZERO: Self = Self(Decimal::ZERO);

  pub fn as_ratio_of<T>(self, other: &Self) -> T
  where
    T: From<Decimal>,
  {
    // Percent::From(Decimal) expects a percentage like 10 for 10%, 15.2 for 15.2%
    // while a division of floats yields a decimal fraction like 0.1 and 0.152
    // Hence the multiplication by 100
    // The T::from is responsible to round up by the DECIMAL resolution.
    T::from(self.0 / other.0 * PERCENT)
  }
}

impl From<i32> for Gram {
  fn from(value: i32) -> Self {
    Gram(Decimal::from_i32(value).unwrap_or(Decimal::ZERO))
  }
}

impl From<f32> for Gram {
  fn from(value: f32) -> Self {
    Gram(Decimal::from_f32(value).unwrap_or(Decimal::ZERO))
  }
}

impl From<usize> for Gram {
  fn from(value: usize) -> Self {
    Gram(Decimal::from_usize(value).unwrap_or(Decimal::ZERO))
  }
}

impl From<Decimal> for Gram {
  fn from(value: Decimal) -> Self {
    Gram(value)
  }
}

// Operator overloading
impl AddAssign<Gram> for Gram {
  fn add_assign(&mut self, other: Self) {
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
    let other: Gram = other.into();
    self + other
  }
}

impl Add<f32> for Gram {
  type Output = Self;
  fn add(self, other: f32) -> Self {
    let other: Gram = other.into();
    self + other
  }
}

// TODO: Best way to plug in Result
impl Sub<Gram> for Gram {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    let diff = self.0 - other.0;
    Gram(diff)
  }
}

impl Sub<i32> for Gram {
  type Output = Self;
  fn sub(self, other: i32) -> Self {
    let other: Gram = other.into();
    self - other
  }
}

impl Sub<f32> for Gram {
  type Output = Self;
  fn sub(self, other: f32) -> Self {
    let other: Gram = other.into();

    self - other
  }
}

impl Mul<Gram> for Gram {
  type Output = Self;
  fn mul(self, other: Gram) -> Self {
    Gram((self.0 * other.0).round_dp(2))
  }
}

impl Mul<i32> for Gram {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    let other: Gram = other.into();
    self * other
  }
}

impl Mul<f32> for Gram {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    let other: Gram = other.into();
    self * other
  }
}

impl Mul<Decimal> for Gram {
  type Output = Self;
  fn mul(self, other: Decimal) -> Self {
    self * Gram(other)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Mul<Percent<MIN, MAX, DECIMALS>>
  for Gram
{
  type Output = Self;
  fn mul(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    self * other.as_decimal()
  }
}

impl Div for Gram {
  type Output = Ratio;
  fn div(self, other: Gram) -> Ratio {
    let other: Gram = other.into();
    (self.0 / other.0).into()
  }
}

impl<T> Div<T> for Gram
where
  T: Into<Gram> + Num,
{
  type Output = Self;
  fn div(self, other: T) -> Self {
    let other: Gram = other.into();
    Gram(self.0 / other.0)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Div<Percent<MIN, MAX, DECIMALS>>
  for Gram
{
  type Output = Self;
  fn div(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    self / other.as_decimal()
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gram_constructor() {
    let weight: Gram = 500.into();
    assert_eq!(weight, 500.into());
  }

  #[test]
  fn test_gram_from_int() {
    let weight: Gram = 312.into();
    assert_eq!(weight, 312.into());
  }

  #[test]
  fn test_gram_from_float() {
    let weight: Gram = 312.5.into();
    assert_eq!(weight, 312.5.into());
  }

  #[test]
  fn test_gram_into_int() {
    let weight: Gram = 11.into();
    assert_eq!(weight, 11.0.into());
  }

  #[test]
  fn test_gram_into_float() {
    let weight: Gram = 11.3.into();
    assert_eq!(weight, 11.3.into());
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
