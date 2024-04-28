use crate::common::Gram;
use crate::common::Percent;
use colored::*;

/// Salt content in dough, range from 1.00%-4.00% with 2 decimal places.
pub type SaltPercentage = Percent<1, 4, 1>;

#[derive(Debug, Clone)]
pub struct Salt {
  pub weight: Gram,
}

impl std::fmt::Display for Salt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", "Salt:".bold().blue(), self.weight.to_string().green().bold())
  }
}
