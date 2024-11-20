use crate::common::mass::*;
use crate::common::percent::*;
use crate::ingredient::flour::Flours;
use crate::ingredient::preferment;
use crate::ingredient::Enrichment;
use crate::ingredient::SaltPercentage;
use crate::ingredient::{Ingredient, Water};
use crate::Cli;
use crate::{Error, Result};
use prettytable::{format, row, Table};
use std::fmt::Debug;
use std::rc::Rc;

pub type Hydration = Percent<50, 120, 0>;

#[derive(Debug)]
pub struct Recipe {
  /// Referece to total flour mass
  total_mass: Rc<Gram>,

  /// A list to all ingredients in recipe
  ingredients: Vec<Box<dyn Ingredient>>,

  /// Required recipe hydration ( Liquid / Total flour mass)
  hydration: Hydration,
}

impl Recipe {
  pub fn new(total_mass: Gram, hydration: Hydration) -> Self {
    Recipe {
      total_mass: Rc::new(total_mass),
      ingredients: vec![],
      hydration: hydration,
    }
  }

  pub fn other(&self) -> Gram {
    self
      .ingredients
      .iter()
      .fold(Gram::ZERO, |a, i| a + i.other())
  }

  pub fn total(&self) -> Gram {
    self
      .ingredients
      .iter()
      .fold(Gram::ZERO, |a, i| a + i.total())
  }

  pub fn flour(&self) -> Gram {
    self
      .ingredients
      .iter()
      .fold(Gram::ZERO, |a, i| a + i.flour())
  }

  /// Returns the total water in recipe
  pub fn water(&self) -> Gram {
    self
      .ingredients
      .iter()
      .fold(Gram::ZERO, |a, i| a + i.water())
  }

  /// The amount of additional water required to achieve the desired hydration
  ///
  ///         <Missing water> =  <Total flour mass> x Hydration - <current water content>
  pub fn missing_water(&self) -> Gram {
    (*self.total_mass * self.hydration) - self.water()
  }

  /// If the requested hydration is not reached
  /// Add appropriate water
  /// If the hydration is already exceeded do nothing
  pub fn add_missing_water(mut self) -> Self {
    let to_add = self.missing_water();
    if to_add > Gram::ZERO {
      self.ingredients.push(Box::new(Water { mass: to_add }));
    }
    self
  }

  pub fn add_generic_enrichment(mut self, name: String, ratio: Option<f32>) -> Self {
    if let Some(ratio) = ratio {
      self
        .ingredients
        .push(Box::new(Enrichment::<SaltPercentage>::new(name, &self.total_mass, ratio.into())));
    }
    self
  }

  pub fn extract_flour_desc(desc: &String) -> Result<(String, i32)> {
    match desc.split_once(':') {
      Some((name, ratio)) => Ok((name.to_owned(), ratio.parse::<i32>().unwrap())),
      _ => Err(Error::InvalidFlourArg(desc.to_owned())),
    }
  }

  pub fn add_flour(&mut self, flours: Vec<String>) -> Result<Flours> {
    flours.iter().fold(
      Ok(Flours::new(&self.total_mass)),
      |acc: Result<Flours>, f| {
        let (name, ratio) = Self::extract_flour_desc(f)?;
        Ok(acc?.add_flour(name, ratio.into()))
      },
    )
  }

  pub fn add_salt(self, ratio: Option<f32> ) -> Self {
    self.add_generic_enrichment("SALT".to_string(), ratio)
  }
  
  pub fn add_preferment(&mut self, preferment: Vec<String>, flours: Flours) -> Result<Flours> {
    preferment.into_iter().fold(Ok(flours), |fs, p| {
      match preferment::BUILDER.get(&p, &self.total_mass) {
        Ok(preferment) => {
          let flours = fs?.repurpose(&*preferment);
          self.ingredients.push(preferment);
          flours
        }
        _ => Err(Error::InvalidPrefermentArgs(p)),
      }
    })
  }

  pub fn add_enrichment_from_cmd(mut self, desc: &String) -> Result<Self> {
    if let Some((name, ratio)) = desc.split_once('%') {
      let ratio: Ratio = ratio.parse::<i32>().unwrap().into();
      self.ingredients.push(Box::new(Enrichment::<Ratio>::new(name.to_owned(), &self.total_mass, ratio)));
      Ok(self)
    } else if let Some((name, mass)) = desc.split_once(':') {
      let mass: Gram = mass.parse::<i32>().unwrap().into();
      self.ingredients.push(Box::new(Enrichment::<Ratio>::new_by_mass(name.to_owned(), &self.total_mass, mass)));
      Ok(self)
    } else  {
     Err(Error::InvalidEnrichmentArg(desc.to_owned()))
    }
  }

  pub fn add_enrichments(self, enrichments: Vec<String>) -> Result<Self> {
    enrichments.into_iter().fold( Ok(self), |s, desc| {
      s?.add_enrichment_from_cmd(&desc)
    })
  }

  pub fn build(cli: Cli) -> Result<Self> {
    let mut recipe = Recipe::new(cli.mass.unwrap().into(), cli.hydration.unwrap().into());
    let flours = recipe.add_flour(cli.flour)?;
    let flours = recipe.add_preferment(cli.preferment, flours)?;
    recipe.ingredients.push(Box::new(flours));
    
    Ok(recipe.add_salt(cli.salt_percentage)
             .add_enrichments(cli.enrichment)?
             .add_missing_water()
      )
  }

  pub fn display(self) -> Result<()> {
    let total = self.total();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.add_row(row![ cbFy =>"#", "", "mass", "%Flour", "%Total", "Comment"]);

    self
      .ingredients
      .iter()
      .fold(table, |t, it| it.describe(t, total))
      .printstd();

    let real_hydration: Hydration = ((self.water().0 / self.total_mass.0) * PERCENT).into();
    println!(
      "{} / {} = {}",
      self.water(),
      *self.total_mass,
      real_hydration
    );

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    println!("PROPERTIES");
    if real_hydration == self.hydration {
      table.add_row(row!["", "HYDRATION", r -> real_hydration]);
    } else {
      table.add_row(row!["", Fr -> "HYDRATION", rFr -> real_hydration, Fr -> "EXPECTED HYDRATION:", Frr -> self.hydration]);
    }
    table.add_row(row!["", "TOTAL FLOUR", r-> self.total_mass ]);
    table.add_row(row!["", "TOTAL WATER", r-> self.water()]);
    table.add_row(row!["", "TOTAL ENRICHMENT*", r-> self.other()]);
    table.add_row(row!["", "TOTAL WEIGHT", r-> total]);
    table.printstd();

    println!("* Enrichement refers to seeds, sugar, butter, etc. While not conservative salt is also counted here");
    Ok(())
  }
}
