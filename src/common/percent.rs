use crate::error::{Error, Result};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::{
  fmt,
  iter::Sum,
  ops::{Add, Div, RangeInclusive},
};

pub const PERCENT: Decimal = dec!(100);

// A limit bounded percentage abstraction.
//
// A Percent is bounded from `MIN` to `MAX` and has `DECIMAL` significat digits after the decimal point
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Percent<const MIN: usize, const MAX: usize, const DECIMALS: usize>(pub usize);

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Percent<MIN, MAX, DECIMALS> {
  pub const ZERO: Self = Self(0);
  pub const MIN: Self = Self(MIN);
  pub const MAX: Self = Self(MAX);
  pub const DECIMALS: usize = DECIMALS;
  pub const DECIMALS_MULTIPLIER: usize = 10usize.pow(DECIMALS as u32);

  pub fn new(value: usize) -> Result<Self> {
    if MIN * Self::DECIMALS_MULTIPLIER <= value && value <= MAX * Self::DECIMALS_MULTIPLIER {
      Ok(Self(value))
    } else {
      Err(Error::InvalidPercentage(value, MIN, MAX))
    }
  }

  pub fn valid_new(value: usize) -> Option<Self> {
    if (MIN * Self::DECIMALS_MULTIPLIER) <= value && value <= (MAX * Self::DECIMALS_MULTIPLIER) {
      return Some(Self(value));
    }
    None
  }

  /// Returns the Percentage as a decimal number
  /// Example 80% => 0.8
  pub fn as_decimal(&self) -> Decimal {
    let mult: Decimal = Percent::<MIN, MAX, DECIMALS>::DECIMALS_MULTIPLIER.into();
    let normalizer = mult * PERCENT;
    Decimal::from_usize(self.0).unwrap_or(Decimal::ZERO) / normalizer
  }

  //returns the valid value inclusive range
  pub fn range() -> RangeInclusive<u32> {
    MIN as u32..=MAX as u32
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<usize>
  for Percent<MIN, MAX, DECIMALS>
{
  fn from(value: usize) -> Self {
    Percent::new(value * Self::DECIMALS_MULTIPLIER).unwrap() // TODO: Result
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<i32>
  for Percent<MIN, MAX, DECIMALS>
{
  fn from(value: i32) -> Self {
    Percent::new(value as usize * Self::DECIMALS_MULTIPLIER).unwrap() // TODO: Result && Maybe TryFrom
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<u32>
  for Percent<MIN, MAX, DECIMALS>
{
  fn from(value: u32) -> Self {
    Percent::new(value as usize * Self::DECIMALS_MULTIPLIER).unwrap() // TODO: Result && Maybe TryFrom
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<f32>
  for Percent<MIN, MAX, DECIMALS>
{
  fn from(value: f32) -> Self {
    let rounding_decimal = 0.5 / 10_f32.powf(DECIMALS as f32);
    Percent::new(((value + rounding_decimal) * Self::DECIMALS_MULTIPLIER as f32) as usize).unwrap()
    // TODO: Result
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> From<Decimal>
  for Percent<MIN, MAX, DECIMALS>
{
  fn from(value: Decimal) -> Self {
    Percent::new(value.to_u64().unwrap() as usize * Self::DECIMALS_MULTIPLIER).unwrap()
    // TODO: Result
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> fmt::Display
  for Percent<MIN, MAX, DECIMALS>
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:.DECIMALS$}%",
      self.0 as f32 / Self::DECIMALS_MULTIPLIER as f32
    )
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Div<Percent<MIN, MAX, DECIMALS>>
  for Decimal
{
  type Output = Self;
  fn div(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    (self / Decimal::from_usize(other.0).unwrap_or(Decimal::ZERO) / PERCENT).round_dp(2)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Add<Percent<MIN, MAX, DECIMALS>>
  for Percent<MIN, MAX, DECIMALS>
{
  type Output = Self;
  fn add(self, other: Percent<MIN, MAX, DECIMALS>) -> Self {
    Self(self.0 + other.0)
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Sum
  for Percent<MIN, MAX, DECIMALS>
{
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Self>,
  {
    iter.fold(Self(0), |acc, x| acc + x)
  }
}
