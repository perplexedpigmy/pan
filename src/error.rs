// use rust_decimal::Decimal;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Percentage value {0} must be between {1} and {2} including")]
    InvalidPercentage(usize, usize, usize),

    // #[error("negative weight not allowed. Subtration {} - {} = {} panics")]
    // NegativeGram(usize, usize, usize),
}