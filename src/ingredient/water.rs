use crate::common::Gram;

use crate::common::Percent;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use colored::*;

pub type HydrationPercentage = Percent<50, 120, 0>;

/// Water(Liquid) content in dough
#[derive(Debug, Clone)]
pub struct Water {
  pub weight: Gram,
}

impl Water {
  pub fn reset(&self, new_weight: Gram) -> Self {
    Self {
      weight: new_weight,
    }
  }
}

impl<T> From<T> for Water
where
  T: Into<Gram>,
{
  fn from(value: T) -> Self {
    Water {
      weight: value.into(),
    }
  }
}

impl std::fmt::Display for Water {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", "Water:".bold().blue(), self.weight.to_string().green().bold())
  }
}

impl Add<Gram> for Water {
  type Output = Self;
  fn add(self, other: Gram) -> Self {
    Water {
      weight: self.weight + other,
    }
  }
}

impl Add<i32> for Water {
  type Output = Self;
  fn add(self, other: i32) -> Self {
    Water {
      weight: self.weight + other as f32,
    }
  }
}

impl Add<f32> for Water {
  type Output = Self;
  fn add(self, other: f32) -> Self {
    Water {
      weight: self.weight + other,
    }
  }
}

impl Mul<i32> for Water {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    Water {
      weight: self.weight * other as f32,
    }
  }
}

impl Mul<f32> for Water {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    Water {
      weight: self.weight * other,
    }
  }
}

impl Div<i32> for Water {
  type Output = Self;
  fn div(self, other: i32) -> Self {
    Water {
      weight: self.weight / other as f32,
    }
  }
}

impl Div<f32> for Water {
  type Output = Self;
  fn div(self, other: f32) -> Self {
    Water {
      weight: self.weight / other,
    }
  }
}

impl PartialEq for Water {
  fn eq(&self, other: &Self) -> bool {
    self.weight == other.weight
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_water_constructor() {
    let water: Water = 500.into();
    assert_eq!(water.weight, 500.into());
  }

  #[test]
  fn test_water_add_gram() {
    let water: Water = 500.into();
    assert_eq!(water.weight + Gram(30.into()), 530.into());
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
