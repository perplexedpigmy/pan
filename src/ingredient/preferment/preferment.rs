use crate::common::Percent;
use crate::ingredient::ingredient::Ingredient;

pub type PortionPercent = Percent<1, 30, 0>;
pub type HydrationPercent = Percent<50, 500, 0>;

/// Preferment is a mixture of dough components that is allowed to ferment before being added to the final bread dough
/// It will have water and flour components in varying ratios as well as some other ingredients
/// A preferment main properties are:
///  1. Hydration percentage (Water/Liquid as percent of flour)
///  2. Portion in dough  (The preferment's flour as percent of the total flour in recipe)
#[allow(dead_code)]
pub trait Preferment: Ingredient {}
