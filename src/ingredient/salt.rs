use super::ingredient::Ingredient;
use crate::common::mass::*;
use crate::common::percent::Percent;
use prettytable::{row, Table};
use std::rc::Rc;

/// Salt content in dough, range from 1.00%-4.00% with 2 decimal places.
pub type SaltPercentage = Percent<1, 4, 1>;

#[derive(Debug, Clone)]
pub struct Salt {
  pub total_mass: Rc<Gram>,
  pub ratio: SaltPercentage,
}

impl Salt {
  pub fn new(total_mass: &Rc<Gram>, ratio: SaltPercentage) -> Self {
    Salt {
      total_mass: total_mass.clone(),
      ratio,
    }
  }
}

impl Ingredient for Salt {
  fn other(&self) -> Gram {
    *self.total_mass * self.ratio
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let other_ratio_flour: Ratio = self.other().as_ratio_of(&self.total_mass);
    let other_ratio_total: Ratio = self.other().as_ratio_of(&total);
    table.add_row(
      row![ b -> "SALT", "", br -> self.other(), br -> other_ratio_flour, rb -> other_ratio_total],
    );
    table
  }
}
