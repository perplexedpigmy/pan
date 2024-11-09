
use crate::common::percent::Percent;

/// Salt content in dough, range from 1.00%-4.00% with 2 decimal places.
pub type SaltPercentage = Percent<1, 4, 1>;