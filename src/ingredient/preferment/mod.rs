mod builder;
/// A preferment is a portion of dough or a mixture of flour and water that is fermented
/// before being added to the final dough. It's used to enhance flavor, texture, and
/// aroma in breads, pastries, and other baked goods.
/// Examples of preferments include
///   sourdough starters,
///   poolish,
///   tangzhong.
///   biga
///   pâte Fermentée,
///   sponge:
///   yudane
///   songe
///
mod preferment;
mod starter;

pub use crate::ingredient::preferment::builder::BUILDER;
pub use crate::ingredient::preferment::starter::Starter;
