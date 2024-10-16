use crate::common::Gram;
use crate::common::Percent;
use colored::*;
use std::ops::Div;
use std::ops::Mul;

pub type Percentage = Percent<1, 30, 0>;
pub type TangzhongHydrationPercentage = Percent<200, 200, 0>;

#[derive(Debug)]
/// A Tangzhong - A water roux or yu-dane is a paste of flour
/// cooked in water or milk to over 65C to gelatinize the staches  
/// the captured water improves shelf life and texture.
/// 
/// Usualy it is 200% hydration, 2 parts water to 1 part flour
///
/// # fields
///
/// * `flour`: The flour content
/// * `water`: The water content
///
#[derive(Clone)]
pub struct Tangzhong {
  flour: Gram,
  water: Gram,
}

static TANGZHONG_HYDRATION: TangzhongHydrationPercentage = Percent(200);
impl Tangzhong {

  fn set(flour: Gram, hydration: TangzhongHydrationPercentage) -> Self {
    let water = flour * hydration;
    Self {
        flour, 
        water,
    }
  }
  /// Creates a Tangzhong 
  /// 
  /// # Arguments:
  /// 
  /// * 'weight': Total weight of the poolish 
  /// 
  /// Poolish defaults to 1 to 1 ratio (100% hydration)
  pub fn new(flour_weight: Gram) -> Self {
    Tangzhong::set(flour_weight, TANGZHONG_HYDRATION)
  } 

  pub fn get_hydration(&self) -> TangzhongHydrationPercentage {
    ((self.water.0 / self.flour.0) * 100f32).into()
  }

  pub fn get_flour_weight(&self) -> Gram {
    self.flour
  }

  pub fn get_water_weight(&self) -> Gram {
    self.water
  }

  pub fn get_total_weight(&self) -> Gram {
    self.flour + self.water
  }
}


impl Mul<i32> for Tangzhong {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    self * other as f32
  }
}

impl Mul<f32> for Tangzhong {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    Tangzhong {
      water: self.water * other,
      flour: self.flour * other,
    }
  }
}

impl Div<i32> for Tangzhong {
  type Output = Self;
  fn div(self, other: i32) -> Self {
    self / other as f32
  }
}

impl Div<f32> for Tangzhong {
  type Output = Self;
  fn div(self, other: f32) -> Self {
    Tangzhong {
      water: self.water / other,
      flour: self.flour / other,
    }
  }
}

impl PartialEq for Tangzhong {
  fn eq(&self, other: &Self) -> bool {
    self.flour == other.flour 
    && self.water == other.water 
  }
}
impl std::fmt::Display for Tangzhong {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\n     {} {}\n     {} {}\n         = {}",
      "Poolish:".bold().blue(),
      "Flour:".bold(),
      self.flour.to_string().green().bold(),
      "Water:".bold(),
      self.water.to_string().green().bold(),
      self.get_total_weight().to_string().yellow().bold()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tangzhong() {
    let tangzhong = Tangzhong::new(50.into());
    
    assert_eq!(tangzhong.get_flour_weight(), 50.into()); // Gram
    assert_eq!(tangzhong.get_water_weight(), 100.into()); // Gram
    assert_eq!(tangzhong.get_hydration(), 200.into()); // Percent
  }
}