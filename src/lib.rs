pub mod ingredient;
pub mod recipe;
mod common;
mod macros;
mod error;

use crate::common::Measure;
use crate::ingredient::flour::Flour;
use crate::ingredient::flour::FlourMix;
use crate::ingredient::salt::SaltPercentage;
use crate::ingredient::starter::StarterHydrationPercentage;
use crate::ingredient::starter::StarterPercentage;
use crate::ingredient::water::HydrationPercentage;
use crate::recipe::{ Adaptations, ResetStarterWeight, ResetWaterWeight};

use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "WEIGHT", default_value = "600")]
  weight: Option<i32>,

  #[arg(short = 'd', long, value_name = "HYDRATION", default_value = "70")]
  hydration: Option<i32>,

  #[arg(short , long, action = clap::ArgAction::Append, default_value = "White:100")]
  flour: Vec<String>,

  #[arg(short = 'y', long, default_value = "100")]
  starter_hydration: Option<i32>,

  #[arg(short = 'p', long, default_value = "10")]
  starter_percentage: Option<i32>,

  #[arg(short, long, default_value = "2")]
  salt_percentage: Option<f32>,

  #[arg(long)]
  reset_starter_weight: Option<f32>,

  #[arg(long)]
  reset_water_weight: Option<f32>,

}

pub struct Config {
  // Initial recipie attributes
  pub flours: FlourMix,
  pub hydration: HydrationPercentage,
  pub starter_hydration: StarterHydrationPercentage,
  pub starter_percentage: StarterPercentage,
  pub salt_percentage: SaltPercentage,

  pub adaptations: Adaptations,
}

pub fn get_args() -> simple_eyre::Result<Config> {
  simple_eyre::install()?;

  let cli = Cli::parse();
  let mut flours = FlourMix::new(cli.weight.unwrap().into());
  cli.flour.iter().for_each(|f| {
    if let Some((name, ratio)) = match f.split_once(':') {
      Some((f, r)) => Some((f, r.parse::<i32>().unwrap())),
      _ => None,
    } {
      flours.add_flour(Flour::new(name, Measure::Ratio(ratio.into())));
    } else {
      panic!("\"{}\" is not a valid flour/ratio syntax.", f);
    }
  });

  let hydration = cli.hydration.unwrap().into();
  let starter_hydration = cli.starter_hydration.unwrap().into();
  let starter_percentage = cli.starter_percentage.unwrap().into();
  let salt_percentage = cli.salt_percentage.unwrap().into();

  let mut adaptations: Adaptations = vec![];
  if let Some(new_starter_weight) = cli.reset_starter_weight {
    adaptations.push( Box::new( ResetStarterWeight { new_starter_weight: new_starter_weight.into() }));
  }

  if let Some(new_water_weight) = cli.reset_water_weight {
    adaptations.push( Box::new( ResetWaterWeight { new_water_weight: new_water_weight.into() }));
  }

  Ok(Config {
    flours,
    hydration,
    starter_hydration,
    starter_percentage,
    salt_percentage,
    
    adaptations,
  })
}
