use super::ingredient::Ingredient;
use crate::common::mass::*;
use num_traits::Num;
use prettytable::{row, Table};
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Water {
  pub mass: Gram,
}

impl Water {}

impl Ingredient for Water {
  fn water(&self) -> Gram {
    self.mass
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let water_ratio_total: Ratio = self.water().as_ratio_of(&total);
    table.add_row(row![ lb -> "WATER", "", rb -> self.water(), cb -> "", rb -> water_ratio_total]);
    table
  }
}

impl<T> From<T> for Water
where
  T: Into<Gram>,
{
  fn from(value: T) -> Self {
    Water { mass: value.into() }
  }
}

impl<T> Add<T> for Water
where
  T: Into<Gram>,
{
  type Output = Self;
  fn add(self, other: T) -> Self {
    Water {
      mass: self.mass + other.into(),
    }
  }
}

impl<T> Mul<T> for Water
where
  T: Into<Gram>,
{
  type Output = Self;
  fn mul(self, other: T) -> Self {
    Water {
      mass: self.mass * other.into(),
    }
  }
}

impl<T> Div<T> for Water
where
  T: Into<Gram> + Num,
{
  type Output = Self;
  fn div(self, other: T) -> Self {
    Water {
      mass: self.mass / other,
    }
  }
}

impl PartialEq for Water {
  fn eq(&self, other: &Self) -> bool {
    self.mass == other.mass
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_water_constructor() {
    let water: Water = 500.into();
    assert_eq!(water.mass, 500.into());
  }

  #[test]
  fn test_water_add_gram() {
    let water: Water = 500.into();
    assert_eq!(water.mass + Gram(30.into()), 530.into());
  }

  #[test]
  fn test_water_add_i32() {
    let water: Water = 500.into();
    assert_eq!(water + 30, 530.into());
  }

  #[test]
  fn test_water_add_f32() {
    let water: Water = 500.into();
    assert_eq!(water + 30.5, 530.5.into());
  }

  #[test]
  fn test_water_mul_i32() {
    let water: Water = 200.into();
    assert_eq!(water * 3, 600.into());
  }

  #[test]
  fn test_water_mul_f32() {
    let water: Water = 200.into();
    assert_eq!(water * 3.5, 700.into());
  }

  #[test]
  fn test_water_div_i32() {
    let water: Water = 600.into();
    assert_eq!(water / 3, 200.into());
  }

  #[test]
  fn test_water_div_f32() {
    let water: Water = 700.into();
    assert_eq!(water / 3.5, 200.into());
  }
}
