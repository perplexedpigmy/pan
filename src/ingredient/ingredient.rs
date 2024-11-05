use prettytable::Table;

use crate::common::mass::*;

use std::fmt::Debug;

/// The two most important ingredients of a recipe ar
/// 1. Flour  - Baker math uses all quantities relative to the total flour content
/// 2. Liquid - Namely water, as the dough's hydration, plays a centre role in the final result
///
/// All the rest of the ingredients are counted as `other`
pub trait Ingredient: Debug {
  /// Water mass of the ingredient
  fn water(&self) -> Gram {
    Gram::ZERO
  }
  // Flour mass of the ingredient
  fn flour(&self) -> Gram {
    Gram::ZERO
  }

  /// The mass of non water/flour(i.e salt, seeds, sugar etc)
  fn other(&self) -> Gram {
    Gram::ZERO
  }

  /// The total mass of the Ingridient
  fn total(&self) -> Gram {
    self.flour() + self.water() + self.other()
  }

  fn describe(&self, table: Table, _: Gram) -> Table {
    table
  }
}
