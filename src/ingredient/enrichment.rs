use super::ingredient::Ingredient;
use crate::common::mass::*;
use prettytable::{row, Table};
use rust_decimal::Decimal;
use std::{ops::Mul, rc::Rc};

#[derive(Debug, Clone)]
pub struct Enrichment<P> {
  pub name: String,
  pub total_mass: Rc<Gram>,
  pub ratio: P,
}

impl<P> Enrichment<P> 
where 
  P: From<usize> + std::fmt::Debug
{
  pub fn new(name: String, total_mass: &Rc<Gram>, ratio: P) -> Self {
    Enrichment {
      name,
      total_mass: total_mass.clone(),
      ratio,
    }
  }

  pub fn new_by_mass(name: String, total_mass: &Rc<Gram>, mass: Gram) -> Self 
  where 
   P: From<Decimal> + std::fmt::Debug + std::fmt::Display + Clone,
   Gram: Mul<P, Output=Gram>,
   {
    // TODO: Formalize the mass & percent algebra
    let ratio: P = (mass.0 / total_mass.0 * Decimal::from(100)).into();
    Self::new(name, total_mass, ratio)
  }
}

impl<P> Ingredient for Enrichment<P> 
where 
  P: std::fmt::Debug + Copy,
  Gram: Mul<P, Output = Gram>,
{
  fn other(&self) -> Gram {
    (*self.total_mass * self.ratio).0.round().into()
  }

  fn describe(&self, mut table: Table, total: Gram) -> Table {
    let other_ratio_flour: Ratio = self.other().as_ratio_of(&self.total_mass);
    let other_ratio_total: Ratio = self.other().as_ratio_of(&total);
    table.add_row(
      row![ b -> self.name.to_uppercase(), "", br -> self.other(), br -> other_ratio_flour, rb -> other_ratio_total],
    );
    table
  }
}
