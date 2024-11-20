use prettytable::{row, Table};
use std::rc::Rc;

use super::Ingredient;
use crate::common::mass::*;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Flour {
  /// Flour designation, anything goes, preferably should be at least unique per recipe
  name: String,

  /// The part, in percentage, of the flour from the flour list in the recipe
  ratio: Ratio,

  /// Flour, in grams, that shouldn't be added, because it is in preferments
  repurposed: Gram,

  /// A Reference to total flour mass in recipe
  total_mass: Rc<Gram>,
}

impl Flour {
  fn new(name: String, ratio: Ratio, total_mass: &Rc<Gram>) -> Self {
    Flour {
      name,
      ratio,
      repurposed: Gram::ZERO,
      total_mass: total_mass.clone(),
    }
  }

  /// The content of the flour in recipe, including the part in preferments
  pub fn gross_flour(&self) -> Gram {
    *self.total_mass * self.ratio
  }
}

impl Ingredient for Flour {
  /// Only the part to be actively added is
  fn flour(&self) -> Gram {
    *self.total_mass * self.ratio - self.repurposed
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let added_ratio_flour: Ratio = self.flour().as_ratio_of(&self.total_mass);
    let added_ratio_total: Ratio = self.flour().as_ratio_of(&total);

    let used = self.gross_flour() - self.flour();
    let used_ratio: Ratio = used.as_ratio_of(&&self.gross_flour());
    let used_comment = if used > Gram::ZERO {
      format!("{}({}) used in preferments", used_ratio, used)
    } else {
      format!("")
    };
    let comment = format!("{} of flour content. {}", self.ratio, used_comment);
    table.add_row(row!["", self.name, r -> self.flour(), r -> added_ratio_flour, r -> added_ratio_total, comment ]);
    table
  }
}

/// A flour aggregator, to enforce certain constraints
#[derive(Debug, Clone)]
pub struct Flours {
  pub total_mass: Rc<Gram>,
  pub mix: Vec<Flour>,
}

impl Ingredient for Flours {
  fn flour(&self) -> Gram {
    self.mix.iter().fold(Gram::ZERO, |a, f| a + f.total())
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let total_gross_flour = self.mix.iter().fold(Gram::ZERO, |a, f| a + f.gross_flour());
    let total_added_flour = self.mix.iter().fold(Gram::ZERO, |a, f| a + f.flour());
    let added_ratio_flour: Ratio = total_added_flour.as_ratio_of(&self.total_mass);
    let added_ratio_total: Ratio = total_added_flour.as_ratio_of(&total);

    let used = total_gross_flour - total_added_flour;
    let used_ratio: Ratio = used.as_ratio_of(&self.total_mass);
    let comment = format!("{}({}) used in preferments", used_ratio, used);
    table.add_row(row![ b -> "FLOUR", "", rb -> total_added_flour, rb -> added_ratio_flour, rb -> added_ratio_total, b -> comment]);
    self.mix.iter().fold(table, |t, f| f.describe(t, total))
  }
}

impl Flours {
  pub fn new(total_mass: &Rc<Gram>) -> Self {
    Flours {
      total_mass: total_mass.clone(),
      mix: Vec::new(),
    }
  }

  /// Add a flour to the flour Mix
  /// Flours can be added by ratio (TODO: Maybe by mass, if intresting)
  pub fn add_flour(mut self, name: String, ratio: Ratio) -> Self {
    let flour = Flour::new(name, ratio, &self.total_mass);

    self.mix.push(flour);
    self
  }

  /// Get the sum of all flour ratios
  pub fn total_ratio(&self) -> Result<Ratio> {
    let total_ratio: Ratio = self.mix.iter().fold(0.into(), |a, f| a + f.ratio);
    if total_ratio != 100.into() {
      Err(Error::InsufficientFLourRatios(total_ratio))
    } else {
      Ok(total_ratio.into())
    }
  }

  /// Removes the part of flour used by preferments from the added flour
  /// Precondition: Should only be called on Flours mix with 100% ratio sum.
  pub fn repurpose(self, ingredient: &dyn Ingredient) -> Result<Self> {
    let mut q = ingredient.flour();
    let mix = self
      .mix
      .into_iter()
      .map(|f| {
        let used = std::cmp::min(f.flour(), q);
        q = q - used;

        Flour {
          name: f.name,
          ratio: f.ratio,
          total_mass: f.total_mass,
          repurposed: f.repurposed + used,
        }
      })
      .collect();

    if q > Gram::ZERO {
      Err(Error::InsufficientFlour(
        *self.total_mass,
        ingredient.flour(),
      ))
    } else {
      Ok(Flours {
        total_mass: self.total_mass,
        mix,
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::ingredient::preferment::Starter;

use super::*;
  use arbtest::arbtest;
  use rand::Rng;

  /// Returns a random sequence of numbers that sums up to `sum`
  fn random_sum(num_elem: u32, sum: u32) -> Vec<u32> {
    let mut vec = vec![0; num_elem as usize];

    let mut remaining_sum = sum;
    for i in 0..num_elem - 1 {
      let random_value = rand::thread_rng().gen_range(1..=remaining_sum - (num_elem - i - 1));
      vec[i as usize] = random_value;
      remaining_sum -= random_value;
    }

    vec[(num_elem - 1) as usize] = remaining_sum;
    vec
  }

  #[test]
  fn valid_when_ratios_sum_to_100() {
    let mass = Rc::new(Gram(1000.into()));

    arbtest(|u| {
      let num_flours: u32 = u.int_in_range(1..=10)?;

      let flours = Flours::new(&mass);

      let flours = random_sum(num_flours, 100)
        .iter()
        .fold(flours, |fs, i| fs.add_flour("f".into(), (*i).into()));

      let result = flours.total_ratio();
      assert_eq!(result.is_ok(), true);
      assert_eq!(result.unwrap(), 100.into());
      Ok(())
    });
  }

  #[test]
  fn invalid_when_ratios_do_not_sum_less_then_100() {
    let mass = Rc::new(Gram(1000.into()));

    arbtest(|u| {
      let num_flours: u32 = u.int_in_range(1..=10)?;
      let sum: u32 = u.int_in_range(20..=99)?;

      let flours = Flours::new(&mass);

      let range = random_sum(num_flours, sum);
      let flours = range
        .iter()
        .fold(flours, |fs, i| fs.add_flour("f".into(), (*i).into()));

      let result = flours.total_ratio();
      assert_eq!(result.is_err(), true);
      assert_eq!(
        result.unwrap_err(),
        Error::InsufficientFLourRatios(sum.into())
      );
      Ok(())
    });
  }

  #[test]
  fn invalid_when_ratios_do_not_sum_greater_than_100() {
    let mass = Rc::new(Gram(1000.into()));

    arbtest(|u| {
      let num_flours: u32 = u.int_in_range(1..=10)?;
      let sum: u32 = u.int_in_range(101..=300)?;

      let flours = Flours::new(&mass);

      let range = random_sum(num_flours, sum);
      let flours = range
        .iter()
        .fold(flours, |fs, i| fs.add_flour("f".into(), (*i).into()));

      let result = flours.total_ratio();
      assert_eq!(result.is_err(), true);
      assert_eq!(
        result.unwrap_err(),
        Error::InsufficientFLourRatios(sum.into())
      );
      Ok(())
    });
  }

  /// Test the invariant
  ///   <Added flour> + <preferment flour> = <Total flour>
  #[test]
  fn valid_repurpose_with_one_flour() {
    arbtest(|u| {
      let portion = u.int_in_range(1..=30)?;
      let mass = Rc::new(Gram(u.int_in_range(500..=5000)?.into()));
      let mut flours = Flours::new(&mass);

      flours = flours.add_flour("f1".into(), 100.into());
      let starter = Starter::new(&mass, portion.into(), 100.into());

      let result = flours.repurpose(&starter);
      assert_eq!(result.is_ok(), true);

      let flours = result.unwrap();
      assert_eq!(flours.mix.len(), 1);

      let flour = &flours.mix[0];
      assert_eq!(flour.gross_flour(), *mass);
      assert_eq!(flour.flour(), *mass - starter.flour());
      Ok(())
    });
  }

  /// Test the invariant
  ///   <Added flour> + <preferment flour> = <Total flour>
  #[test]
  fn valid_repurpose_with_multi_flour() {
    arbtest(|u| {
      let num_flours = u.int_in_range(1..=10)?;
      let portion = u.int_in_range(1..=30)?;
      let mass = Rc::new(Gram(u.int_in_range(500..=5000)?.into()));
      let flours = Flours::new(&mass);

      let range = random_sum(num_flours, 100);
      let flours = range
        .iter()
        .fold(flours, |fs, i| fs.add_flour("f".into(), (*i).into()));

      let starter = Starter::new(&mass, portion.into(), 100.into());

      let result = flours.repurpose(&starter);
      assert_eq!(result.is_ok(), true);

      let flours = result.unwrap();
      assert_eq!(flours.mix.len(), num_flours as usize);

      let mut added_flour = Gram::ZERO;
      flours.mix.iter().for_each(|f| {
        assert_eq!(f.gross_flour(), *mass * f.ratio);
        added_flour = added_flour + f.flour();
      });
      assert_eq!(added_flour, *mass - starter.flour());
      Ok(())
    });
  }

  /// Test the invariant
  ///   <Added flour> + <preferment flour> = <Total flour>
  #[test]
  fn valid_repurpose_with_multi_flour_mutli_preferment() {
    arbtest(|u| {
      let num_flours = u.int_in_range(1..=10)?;
      let portion1 = u.int_in_range(1..=30)?;
      let portion2 = u.int_in_range(1..=30)?;
      let mass = Rc::new(Gram(u.int_in_range(500..=5000)?.into()));
      let flours = Flours::new(&mass);

      let range = random_sum(num_flours, 100);
      let flours = range
        .iter()
        .fold(flours, |fs, i| fs.add_flour("f".into(), (*i).into()));

      let starter = Starter::new(&mass, portion1.into(), 100.into());
      let tangzhong = Starter::new(&mass, portion2.into(), 100.into());

      let result = flours.repurpose(&starter);
      assert_eq!(result.is_ok(), true);
      let result = result.unwrap().repurpose(&tangzhong);
      assert_eq!(result.is_ok(), true);

      let flours = result.unwrap();
      assert_eq!(flours.mix.len(), num_flours as usize);

      let mut added_flour = Gram::ZERO;
      flours.mix.iter().for_each(|f| {
        assert_eq!(f.gross_flour(), *mass * f.ratio);
        added_flour = added_flour + f.flour();
      });
      assert_eq!(added_flour, *mass - (starter.flour() + tangzhong.flour()));
      Ok(())
    });
  }
}
