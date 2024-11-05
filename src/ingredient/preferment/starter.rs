use std::rc::Rc;

use super::preferment::*;
use crate::common::mass::Ratio;
use crate::{common::Gram, ingredient::Ingredient, Error, Result};

use prettytable::{row, Table};

/// Definitions
///   Starter - Long term culture
///   Levain  - A short term culture, derived from Starter, for a specific bake
///
/// The above distinction is irrelevant for our purposes and the following defintion is used
///  sourdough starter is a live culture of wild yeast and bacteria,
///  created by fermenting a mixture of flour and water. It's used to leaven and flavor sourdough bread.
///
/// The abstraction is describing a Starter as
/// - A Flour content as a percent of the total flour content
/// - Hyndration percent (Usually 100%, meaning 1:1 flour to water. )
///
#[derive(Debug)]
pub struct Starter {
  total_flour: Rc<Gram>,
  portion: PortionPercent,
  hydration: HydrationPercent,
}

impl<'a> Starter {
  pub fn new(total_flour: &Rc<Gram>, portion: PortionPercent, hydration: HydrationPercent) -> Self {
    Starter {
      total_flour: total_flour.clone(),
      portion,
      hydration,
    }
  }

  pub fn build(args: String, total_mass: &Rc<Gram>) -> Result<Box<dyn Ingredient>> {
    if let [ratio, hydration] = args.split(':').collect::<Vec<&str>>().as_slice() {
      Ok(Box::new(Starter::new(
        total_mass,
        ratio.parse::<i32>().unwrap().into(),
        hydration.parse::<i32>().unwrap().into(),
      )))
    } else {
      Err(Error::InvalidStarterArgs(args))
    }
  }
}

impl<'a> Ingredient for Starter {
  fn water(&self) -> Gram {
    self.flour() * self.hydration
  }

  fn flour(&self) -> Gram {
    *self.total_flour * self.portion
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let flour_ratio_flour: Ratio = self.flour().as_ratio_of(&self.total_flour);

    let total_ratio_total: Ratio = self.total().as_ratio_of(&total);
    let flour_ratio_total: Ratio = self.flour().as_ratio_of(&total);
    let water_ratio_total: Ratio = self.water().as_ratio_of(&total);

    let comment = format!(
      "{} Hydration, {} of total flour",
      self.hydration, self.portion
    );
    table.add_row(row![lb -> "STARTER", "",            rb -> self.total(),  cb -> "",  rb -> total_ratio_total, b -> comment]);
    table.add_row(row!["",             l -> "flour",  r   -> self.flour(),  r -> flour_ratio_flour, r -> flour_ratio_total]);
    table.add_row(
      row!["",             l -> "water",  r   -> self.water(),  c -> "", r -> water_ratio_total],
    );
    table
  }
}

impl<'a> Preferment for Starter {}

#[cfg(test)]
mod tests {
  use super::*;
  use arbtest::arbtest;

  #[test]
  fn valid_ratio_to_mass() {
    arbtest(|u| {
      let total_flour: Rc<Gram> = Rc::new(u.int_in_range(10..=5000)?.into());
      let portion: PortionPercent = u.int_in_range(PortionPercent::range())?.into();
      let hydration: HydrationPercent = u.int_in_range(HydrationPercent::range())?.into();

      let starter = Starter::new(&total_flour, portion, hydration);
      let starter_flour = *total_flour * portion;
      let starter_water = *total_flour * portion * hydration;

      assert_eq!(starter.flour(), starter_flour);
      assert_eq!(starter.water(), starter_water);
      assert_eq!(starter.total(), starter_flour + starter_water);

      Ok(())
    });
  }
}
