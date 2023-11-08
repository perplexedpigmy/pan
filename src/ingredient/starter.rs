use crate::common::Gram;
use crate::common::Percent;
use crate::ingredient::flour::Flour;
use crate::ingredient::flour::Measure;
use crate::ingredient::water::Water;
use colored::*;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

pub type StarterPercentage = Percent<1, 30, 0>;
pub type StarterHydrationPercentage = Percent<50, 300, 0>;

#[derive(Debug)]
/// A sourdough starter
///
/// Sourdough is composed of flour and water and a hefty amount of healthy lactobacillus bacteria.
/// The flour and water are tracked, the bacteria takes care of itself.
///
/// The parameters thet make affect the effectiveness and properties of the starter are
/// - Hydration
/// - Flour/s used
/// - Temperture
/// - Environment
///
/// The above has effect on the bateria/yiest profile of the starter, but this abstraction only
/// trackes hydration which is the ratio of water to flour.
///
/// # fields
///
/// * `flour`: The flour content
/// * `water`: The water content
///
///
pub struct Starter {
  flour: Gram,
  water: Gram,
}

impl Starter {

  pub fn set(weight: Gram, hydration: StarterHydrationPercentage) -> Starter {

    let flour_ratio = 1.0 / hydration; // The flour ratio for each 1 unit of water
    let portion = weight / (flour_ratio + 1.0);

    let flour = portion * flour_ratio;
    let water = portion;

    Self {
        flour, 
        water
    }
  } 
  /**
   *
   * @param total_flour(f32) total flour weight in grams
   * @param hydration(f32) hydration percentage
   * @param percent_starter(f32) percentage of starter flour from total flour
   */
  /// Create a new starter
  ///
  /// # Fields
  ///
  /// `total_weight`: Total desired weight of flour & water (no salt nor additional ingredients)
  /// `hydration`: Starter required hydration
  /// `percent_starter`: Percent of starter from total_weight``
  ///
  /// # Example
  ///
  /// let starter = Starter::create(200.0, 0.7, 0.1);
  /// ```
  /// assert_eq!(1, 1)
  /// ````
  pub fn create(
    total_weight: Gram,
    hydration: StarterHydrationPercentage,
    percent_starter: StarterPercentage,
  ) -> Starter {
    let starter_weight = total_weight * percent_starter;
    Self::set(starter_weight, hydration)
    // let flour_ratio = 1.0 / hydration; // The flour ratio for each 1 unit of water
    // let portion = starter_weight / (flour_ratio + 1.0);

    // let flour = portion * flour_ratio;
    // let water = portion;

    // Starter { flour, water }
  }

  pub fn get_hydration(&self) -> StarterHydrationPercentage {
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

  pub fn reset(&self, weight: Gram, hydration: StarterHydrationPercentage) -> Starter {
    Self::set(weight, hydration)
  } 
}

impl Add<Water> for Starter {
  type Output = Self;
  fn add(self, other: Water) -> Self {
    Starter {
      water: self.water + other.weight,
      flour: self.flour,
    }
  }
}

/// Adding a Flour depends on the flour measurement
/// When it measures weight the weight is simply added
/// When it measures a ratio the current starter weight is added the ratio's percentage
impl Add<Flour> for Starter {
  type Output = Self;
  fn add(self, other: Flour) -> Self {
    Starter {
      water: self.water,
      flour: self.flour
        + match other.measure {
          Measure::Weight(w) => w,
          Measure::Ratio(r) => self.flour * (r + 100.into()) / 100,
        },
    }
  }
}

impl Mul<i32> for Starter {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    self * other as f32
  }
}

impl Mul<f32> for Starter {
  type Output = Self;
  fn mul(self, other: f32) -> Self {
    Starter {
      water: self.water * other,
      flour: self.flour * other,
    }
  }
}

impl Div<i32> for Starter {
  type Output = Self;
  fn div(self, other: i32) -> Self {
    self / other as f32
  }
}

impl Div<f32> for Starter {
  type Output = Self;
  fn div(self, other: f32) -> Self {
    Starter {
      water: self.water / other,
      flour: self.flour / other,
    }
  }
}

impl PartialEq for Starter {
  fn eq(&self, other: &Self) -> bool {
    self.flour == other.flour && self.water == other.water
  }
}
impl std::fmt::Display for Starter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{} {}):\n     {} {}\n     {} {}\n         = {}",
      "Starter(".bold().blue(),
      self.get_hydration().to_string().bold().blue(),
      "hydration):".bold().blue(),
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
  const FLOUR_TYPE: &str = "TEST";
  use super::*;

  #[test]
  fn test_flour_constructor() {
    let flour = Flour {
      name: FLOUR_TYPE.into(),
      measure: Measure::Weight(500.into()),
      weight: None,
    };
    assert_eq!(flour.name, FLOUR_TYPE);
    assert_eq!(flour.measure, Measure::Weight(500.into()));
  }
}
