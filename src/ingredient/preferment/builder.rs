use crate::{
  common::Gram,
  ingredient::{preferment::Starter, Ingredient},
  Error, Result,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, rc::Rc};

type StringToBuilder = fn(String, &Rc<Gram>) -> Result<Box<dyn Ingredient>>;
pub struct Builder {
  builders: HashMap<String, StringToBuilder>,
}

impl Builder {
  fn new(builders: Vec<(String, StringToBuilder)>) -> Self {
    Builder {
      builders: builders.into_iter().collect(),
    }
  }

  /// Returns (ID, ARGS) pair extracted from the user's prefermant description
  fn extract_args(desc: &String) -> Result<(String, String)> {
    match desc.split_once(':') {
      Some((id, args)) => Ok((id.to_owned(), args.to_owned())),
      _ => Err(Error::InvalidPrefermentArgs(desc.clone())),
    }
  }

  pub fn get(&self, desc: &String, total_flour: &Rc<Gram>) -> Result<Box<dyn Ingredient>> {
    let (id, args) = Self::extract_args(desc)?;
    match self.builders.get(&id.to_lowercase()) {
      Some(builder) => builder(args, total_flour),
      _ => Err(Error::InvalidPreferment(id)),
    }
  }
}

lazy_static! {
  pub static ref BUILDER: Builder = Builder::new(vec!(("starter".to_owned(), Starter::build)),);
}

// pub fn build_preferment(desc: String) -> Result<Box<dyn Preferment>> {
//   if let Some((id, rest)) = match desc.split_once(':') {
//   } else {
//     Err(Error::InvalidPreferment(desc))
//   }
// }
