use simple_eyre::eyre::{eyre, Result};
use clap::{Parser, command,arg };
use crate::ingredient::flour::FlourItem;



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

    #[arg(short , long, action = clap::ArgAction::Append, default_value = "White:100")]
    flour: Vec<String>,
}

pub struct Args {
    pub weight: i32,
    pub flours: Vec<FlourItem>,
}

pub fn get_args() -> Result<Args> {
    simple_eyre::install()?;

    let cli = Cli::parse();
    println!("{:#?}", cli.weight);
    let mut total_percentage = 0;
    let flours : Vec<FlourItem>= cli.flour.iter().map( |f| {
        if let Some((flour, ratio)) = match f.split_once(":") {
            Some((f, r)) => Some(( f, r.parse::<i32>().unwrap())),
            _ => None,
        } {
            total_percentage += ratio;
            FlourItem {
                name: flour.to_string(),
                percentage: ratio.into()
            }
        } else {
            panic!("\"{}\" is not a valid flour/ratio syntax.", f);
        }
    }).collect(); 

    if total_percentage < 100 || total_percentage > 100 {
        return Err(eyre!("Total flour percentage must but be equal to 100%"));
    }
        
    println!("{:#?}", flours);

    Ok(Args {
        weight: cli.weight.unwrap(), 
        flours,
    }) 
}

