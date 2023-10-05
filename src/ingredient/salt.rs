use crate::common::Gram;
use crate::common::Percent;
use colored::*;

pub type SaltPercentage = Percent<1, 4>;

#[derive(Debug)]
pub struct Salt {
  pub weight: Gram,
}

impl std::fmt::Display for Salt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", "Salt:".bold().blue(), self.weight.to_string().green().bold())
  }
}
