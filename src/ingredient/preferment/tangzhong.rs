use std::rc::Rc;

use super::preferment::*;
use crate::common::mass::Ratio;
use crate::{common::Gram, ingredient::Ingredient, Error, Result};

use prettytable::{row, Table};

/// Tangzhong, also known as a water roux, is a Japanese technique that involves cooking 
/// a small amount of flour and water to create a paste. This paste is then added to bread dough, 
/// resulting in a softer, moister, and more flavorful bread.
///
/// Benefits of using Tangzhong:
///   Improved Texture: The paste helps to retain moisture in the bread, resulting in a softer and more tender crumb.
///   Enhanced Flavor: The cooking process  gelatinizes the starch in the flour (at about >63c) and malt creation
///   Longer Shelf Life: Due to higher hydration, Tangzhong bread tends to stay fresh for longer.
/// 
/// How to use Tangzhong:
///  Mixing one part flour to at least 1 part and up to 5 parts boiling water, This gelatinazies most of the flour, 
///  allowing it to capture the water. After cooling allow the tangzhong to rest at least 12 hours in the fridge, 
///  The chemical process breaks to starches to malt making it slightly sweet
///  
///
#[derive(Debug)]
pub struct Tangzhong {
  total_flour: Rc<Gram>,
  portion: PortionPercent,
  hydration: HydrationPercent,
}

impl<'a> Tangzhong {
  pub fn new(total_flour: &Rc<Gram>, portion: PortionPercent, hydration: HydrationPercent) -> Self {
    Tangzhong {
      total_flour: total_flour.clone(),
      portion,
      hydration,
    }
  }

  pub fn build(args: String, total_mass: &Rc<Gram>) -> Result<Box<dyn Ingredient>> {
    if let [ratio, hydration] = args.split(':').collect::<Vec<&str>>().as_slice() {
      Ok(Box::new(Tangzhong::new(
        total_mass,
        ratio.parse::<i32>().unwrap().into(),
        hydration.parse::<i32>().unwrap().into(),
      )))
    } else {
      Err(Error::InvalidStarterArgs(args))
    }
  }
}

impl<'a> Ingredient for Tangzhong {
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
    table.add_row(row![lb -> "TANGZHONG", "",            rb -> self.total(),  cb -> "",  rb -> total_ratio_total, b -> comment]);
    table.add_row(row!["",                l -> "flour",  r   -> self.flour(),  r -> flour_ratio_flour, r -> flour_ratio_total]);
    table.add_row(
      row!["",             l -> "water",  r   -> self.water(),  c -> "", r -> water_ratio_total],
    );
    table
  }
}

impl<'a> Preferment for Tangzhong {}

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

      let starter = Tangzhong::new(&total_flour, portion, hydration);
      let starter_flour = *total_flour * portion;
      let starter_water = *total_flour * portion * hydration;

      assert_eq!(starter.flour(), starter_flour);
      assert_eq!(starter.water(), starter_water);
      assert_eq!(starter.total(), starter_flour + starter_water);

      Ok(())
    });
  }
}
