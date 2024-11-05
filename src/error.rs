// use rust_decimal::Decimal;
use thiserror::Error;

use crate::common::{mass::Ratio, Gram};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
  #[error("Percentage value {0} must be between {1} and {2} including")]
  InvalidPercentage(usize, usize, usize),

  #[error("Invalid flour:ratio format {0}")]
  InvalidFlourArg(String),

  #[error("Invalid <preferment name>:<colon separated additional args> format {0}")]
  InvalidPrefermentArgs(String),

  #[error("Preferment '{0}' not supported{0}")]
  InvalidPreferment(String),

  #[error("Not enough flour to be used by preferment, total flour {0}g requested preferment {1}g")]
  InsufficientFlour(Gram, Gram),

  #[error("A total description of 100% of the four is required got {0}%")]
  InsufficientFLourRatios(Ratio),

  #[error("Starter expects <ratio>:<Hydration> Arguments. Got {0}")]
  InvalidStarterArgs(String),
}
