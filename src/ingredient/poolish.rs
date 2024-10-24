use crate::common::Gram;
use crate::common::Percent;
use colored::*;
use std::ops::Div;
use std::ops::Mul;

pub type Percentage = Percent<1, 30, 0>;
pub type PoolishHydrationPercentage = Percent<50, 300, 0>;

#[derive(Debug)]
/// A Poolish pre ferment used in bread and Pizza making 
/// 
/// Usualy it has 100% hydration, 1-to-1 unit of flour/water
///
/// # fields
///
/// * `flour`: The flour content
/// * `water`: The water content
///
///
#[derive(Clone)]
pub struct Poolish {
  flour: Gram,
  water: Gram,
  yeast: Gram,
}

static POOLISH_HYDRATION: PoolishHydrationPercentage = Percent(100);
impl Poolish {

  fn set(weight: Gram, hydration: PoolishHydrationPercentage) -> Self {
    let flour_ratio: PoolishHydrationPercentage = (1usize / hydration.0).into(); 
    let portion = weight / (flour_ratio + 1.into());

    let flour = portion * flour_ratio;
    let water = portion;
    let yeast = water * 0.01;

    Self {
        flour, 
        water,
        yeast,
    }

  }
  /// Creates a Poolish
  /// 
  /// # Arguments:
  /// 
  /// * 'weight': Total weight of the poolish 
  /// 
  /// Poolish defaults to 1 to 1 ratio (100% hydration)
  pub fn new(weight: Gram) -> Self {
    Poolish::set(weight, POOLISH_HYDRATION)
  } 

  #[allow(unused)]
  fn with_hydration(self, hydration: PoolishHydrationPercentage) -> Self {
    Poolish::set(self.get_total_weight() - self.yeast, hydration)
  }

  pub fn get_hydration(&self) -> PoolishHydrationPercentage {
    // ((self.water / self.flour) * PERCENT).into()
    0.into()
  }

  pub fn get_flour_weight(&self) -> Gram {
    self.flour
  }

  pub fn get_water_weight(&self) -> Gram {
    self.water
  }

  pub fn get_yeast_weight(&self) -> Gram {
    self.yeast
  }

  pub fn get_total_weight(&self) -> Gram {
    self.flour + self.water + self.yeast
  }

}


impl Mul<i32> for Poolish {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    self * other as f32
  }
}

impl Mul<f32> for Poolish {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    Poolish {
      water: self.water * other,
      flour: self.flour * other,
      yeast: self.yeast * other,
    }
  }
}

impl Div<i32> for Poolish {
  type Output = Self;
  fn div(self, other: i32) -> Self {
    self / other as f32
  }
}

impl Div<f32> for Poolish {
  type Output = Self;
  fn div(self, other: f32) -> Self {
    Poolish {
      water: self.water / other,
      flour: self.flour / other,
      yeast: self.yeast / other,
    }
  }
}

impl PartialEq for Poolish {
  fn eq(&self, other: &Self) -> bool {
    self.flour == other.flour 
    && self.water == other.water 
    && self.yeast == other.yeast
  }
}
impl std::fmt::Display for Poolish {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{} {}\n     {} {}\n     {} {}\n      {} {}\n         = {}",
      "Poolish(".bold().blue(),
      self.get_hydration().to_string().bold().blue(),
      "hydration):".bold().blue(),
      "Flour:".bold(),
      self.flour.to_string().green().bold(),
      "Water:".bold(),
      self.water.to_string().green().bold(),
      "Yeast:".bold(),
      self.yeast.to_string().green().bold(),
      self.get_total_weight().to_string().yellow().bold()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_() {
    let poolish = Poolish::new(200.into());
    
    assert_eq!(poolish.get_flour_weight(), 100.into()); // Gram
    assert_eq!(poolish.get_water_weight(), 100.into()); // Gram
    assert_eq!(poolish.get_yeast_weight(), 1.0.into()); // Gram
    assert_eq!(poolish.get_hydration(), 100.into()); // Percent
  }

  #[test]
  fn test_poolish_hydration() {
    let poolish = Poolish::new(200.0.into())
    .with_hydration(80.into());
    println!("{}", poolish);
  }    
}