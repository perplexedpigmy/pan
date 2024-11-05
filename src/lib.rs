mod common;
mod error;
mod ingredient;

mod macros;
pub mod recipe;

use crate::error::{Error, Result};
use clap::Parser;

// pub type Hydration = Percent<50, 120, 0>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[arg(
    short,
    long,
    value_name = "MASS",
    default_value = "600",
    help = "Total flour mass in gram. Default: 600"
  )]
  mass: Option<i32>,

  #[arg(short = 'd', long, value_name = "HYDRATION", default_value = "70")]
  hydration: Option<i32>,

  #[arg(short , long, action = clap::ArgAction::Append, default_value = "White:100", help="Flour name:Percentage. Default: White:100")]
  flour: Vec<String>,

  #[arg(
    short,
    long,
    default_value = "2",
    help = "Salt content as percentage of flour, Default 2%"
  )]
  salt_percentage: Option<f32>,

  // -- Preferments
  #[arg(
    short = 'p',
    long,
    action = clap::ArgAction::Append,
    help = "Preferments to use syntax <name>:<ratio of flour>:<hydration>. Example: starter:10:100. for 100% hydrated sourdough starter as 10% of flour mass"
  )]
  preferment: Vec<String>,
  // -- Transformations
  // #[arg(long)]
  // reset_starter_weight: Option<f32>,

  // #[arg(long)]
  // reset_water_weight: Option<f32>,
}

// #[derive(Debug)]
// pub struct Recipe {
//   /// Referece to total flour mass
//   total_mass: Rc<Gram>,

//   /// A list to all ingredients in recipe
//   ingredients: Vec<Box<dyn Ingredient>>,

//   /// Required recipe hydration ( Liquid / Total flour mass)
//   hydration: Hydration,
// }

// impl Recipe {
//   fn new(total_mass: Gram, hydration: Hydration) -> Self {
//     Recipe {
//       total_mass: Rc::new(total_mass),
//       ingredients: vec![],
//       hydration: hydration,
//     }
//   }

//   /// Returns the total water in recipe
//   fn water(&self) -> Gram {
//     self.ingredients.iter().fold(Gram::ZERO, |a, i|
//       a + i.water()
//     )
//   }

//   /// The amount of additional water required to achieve the desired hydration
//   ///
//   ///         <Missing water> =  <Total flour mass> x Hydration - <current water content>
//   fn missing_water(&self) -> Gram {
//     (*self.total_mass * self.hydration) - self.water()
//   }

//   /// If the requested hydration is not reached
//   /// Add appropriate water
//   /// If the hydration is already exceeded do nothing
//   fn add_missing_water(&mut self) -> &mut Self{
//     let to_add = self.missing_water();
//     if to_add > Gram::ZERO {
//       self.ingredients.push(Box::new( Water { mass: to_add }));
//     }
//     self
//   }

// }

pub fn get_args() -> Result<Cli> {
  Ok(Cli::parse())
}
