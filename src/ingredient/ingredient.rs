
use crate::{/*common::Gram ,*/ ingredient::{flour::FlourMix, salt::Salt, starter::Starter, water::Water}};


// trait Ingredient2 {
//   fn get_water(&self) -> Gram { Gram::ZERO }
//   fn get_flour(&self) -> Gram { Gram::ZERO }
// }

#[derive(Debug, Clone)]
pub enum Ingredient {
  Water(Water),
  Flour(FlourMix),
  Salt(Salt),
  Starter(Starter),
}

impl std::fmt::Display for Ingredient {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Ingredient::Water(w) => w.to_string(),
        Ingredient::Flour(fl) => fl.to_string(),
        Ingredient::Salt(s) => s.to_string(),
        Ingredient::Starter(s) => s.to_string(),
      }
    )
  }
}
