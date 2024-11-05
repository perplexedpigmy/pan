pub mod mass;
pub mod percent;

pub use crate::common::mass::Gram;
pub use crate::common::percent::Percent;

////////////////////////////////////////////
// TODO: Does measure serves anything
/*
#[derive(Debug, Clone, PartialEq)]
pub enum  Measure<const MIN: usize, const MAX: usize, const DECIMALS: usize>  {
  Weight(Gram),
  Ratio(Percent<MIN, MAX, DECIMALS>),
}
*/
