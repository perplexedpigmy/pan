use crate::common::Gram;
use crate::Config;
use crate::ingredient::{
    salt::Salt,
    water::Water,
    salt::SaltPercentage,
    water::HydrationPercentage,
    starter::Starter,
    starter::StarterPercentage,
    ingredient::Ingredient,
};
// use colorize::AnsiColor;

#[derive(Debug)]
pub struct Recipe {
    total_weight: Gram,
    hydration_percentage: HydrationPercentage,
    total_flour_weight: Gram,
    total_water_weight: Gram,
    salt_weight: Gram,
    salt_percentage: SaltPercentage,
    starter_percentage: StarterPercentage,

    ingredients: Vec<Ingredient>,
}
use colored::*;

impl Recipe {
    
    pub fn get_starter(&self) -> Option<&Starter> {
        for ingredient in &self.ingredients {
            if let Ingredient::Starter(s) = ingredient {
                return Some(s);
            }
        }
        None
    }

    pub fn craft_by_ratio(total_flour_weight: &Gram, config: Config) -> Self {
        let total_water_weight = *total_flour_weight * config.hydration;
        let starter = Starter::create(*total_flour_weight + total_water_weight, config.starter_hydration, config.starter_percentage);
        let salt_weight = *total_flour_weight * config.salt_percentage;
        let total_weight = *total_flour_weight + total_water_weight + salt_weight;

        Recipe {
            total_weight,
            salt_weight,
            salt_percentage: config.salt_percentage,
            hydration_percentage: config.hydration,
            total_flour_weight: *total_flour_weight,
            total_water_weight,
            starter_percentage: config.starter_percentage,
            ingredients: vec![
                Ingredient::Flour(config.flours.apply_starter(&starter)),
                Ingredient::Water(Water { weight: total_water_weight.into() }),
                Ingredient::Salt(Salt { weight: salt_weight.into() }),
                Ingredient::Starter(starter),
            ]
        }
    }

    fn craft_by_weight(_config: Config) -> Self {
       unimplemented!("Only by ratio is supported!"); 
    }

    pub fn craft(config: Config) -> Self {
        match config.flours.total_weight {
            Some(tw) => Self::craft_by_ratio(&tw, config),
            None     => Self::craft_by_weight(config),
        }
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Summary:\n Total weight: {}\n Flour: {}\n Water({}): {}\n Salt({}): {}\n\nIngredients: (using {} starter)\n   {}", 
            self.total_weight,
            self.total_flour_weight,
            self.hydration_percentage,
            self.total_water_weight,
            self.salt_percentage,
            self.salt_weight,
            self.starter_percentage,
            self.ingredients.iter()
                 .map(|i| i.to_string()).collect::<Vec<String>>()
                 .join("\n   ")
        )
    }
}
