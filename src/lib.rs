use crate::ingredient::flour::Flour;
use crate::ingredient::flour::FlourMix;
use crate::ingredient::flour::Measure;
use crate::ingredient::salt::SaltPercentage;
use crate::ingredient::starter::StarterHydrationPercentage;
use crate::ingredient::starter::StarterPercentage;
use crate::ingredient::water::HydrationPercentage;
use clap::{arg, command, Parser};
use simple_eyre::eyre::Result;

#[macro_use]
mod macros;
mod common;
pub mod ingredient;
pub mod recipe;

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
}

pub struct Config {
  pub flours: FlourMix,
  pub hydration: HydrationPercentage,
  pub starter_hydration: StarterHydrationPercentage,
  pub starter_percentage: StarterPercentage,
  pub salt_percentage: SaltPercentage,
}

pub fn get_args() -> Result<Config> {
  simple_eyre::install()?;

  let cli = Cli::parse();
  let mut flours = FlourMix::new(cli.weight.unwrap().into());
  cli.flour.iter().for_each(|f| {
    if let Some((name, ratio)) = match f.split_once(":") {
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

  Ok(Config {
    flours,
    hydration,
    starter_hydration,
    starter_percentage,
    salt_percentage,
  })
}
