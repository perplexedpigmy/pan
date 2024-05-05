use crate::common::Gram;
use crate::ingredient::{
  ingredient::Ingredient, salt::Salt, salt::SaltPercentage, starter::Starter,
  starter::StarterPercentage, starter::StarterHydrationPercentage, water::HydrationPercentage, water::Water,
};
use crate::Config;
use std::fmt::Debug;
use crate::replace_element;
use crate::remove_element;

pub trait Adaptation : Debug {
  fn adapt_by_weight(&self, recipe: &mut Recipe) -> simple_eyre::Result<Recipe>;
}
pub type Adaptations = Vec<Box<dyn Adaptation>>;

#[derive(Debug, Clone)]
pub struct ResetStarterWeight {
  pub new_starter_weight: Gram,
}

impl Adaptation for ResetStarterWeight {
  fn adapt_by_weight(&self, recipe: &mut Recipe) -> simple_eyre::Result<Recipe> {
    let starter_hydration = recipe.get_starter().ok_or::<StarterHydrationPercentage>(100.into()).unwrap().get_hydration();
    recipe.set_starter_weight(self.new_starter_weight.into(), starter_hydration)
  }
}

#[derive(Debug, Clone)]
pub struct ResetWaterWeight {
  pub new_water_weight: Gram,
}

impl Adaptation for ResetWaterWeight {
  fn adapt_by_weight(&self, recipe: &mut Recipe) -> simple_eyre::Result<Recipe> {
    recipe.set_water_weight(self.new_water_weight.into())
  }
}


#[derive(Debug, Clone)]
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

  // pub fn remove<T,F>(vec: &mut Vec<T>, condition: F)
  // where
  //   F: Fn(&T) -> bool,
  // {
  //   vec.retain(|i| !condition(i))
  // }
 
  pub fn get_starter(&self) -> Option<&Starter> {
    for ingredient in &self.ingredients {
      if let Ingredient::Starter(s) = ingredient {
        return Some(s);
      }
    }
    None
  }

  pub fn get_water(&self) -> Option<&Water> {
    for ingredient in &self.ingredients {
      if let Ingredient::Water(s) = ingredient {
        return Some(s);
      }
    }
    None
  }


  pub fn craft_by_ratio(total_flour_weight: &Gram, config: Config) -> simple_eyre::Result<(Self, Adaptations)> {
    let total_water_weight = *total_flour_weight * config.hydration;
    let starter = Starter::create(
      *total_flour_weight + total_water_weight,
      config.starter_hydration,
      config.starter_percentage,
    );
   let salt_weight = *total_flour_weight * config.salt_percentage;
   let total_weight = *total_flour_weight + total_water_weight + salt_weight;

    Ok((Recipe {
      total_weight,
      salt_weight,
      salt_percentage: config.salt_percentage,
      hydration_percentage: config.hydration,
      total_flour_weight: *total_flour_weight,
      total_water_weight,
      starter_percentage: config.starter_percentage,
      ingredients: vec![
        Ingredient::Flour(config.flours.apply_starter(&starter)),
        Ingredient::Water(Water {
          weight: total_water_weight - starter.get_water_weight()
        }),
        Ingredient::Salt(Salt {
          weight: salt_weight
        }),
        Ingredient::Starter(starter),
      ],
    }, config.adaptations))
  }

  fn craft_by_weight(_config: Config) -> simple_eyre::Result<(Self, Adaptations)> {
    unimplemented!("Recipe composition by weight is not supported!");
  }

  pub fn adapt(recipe: (Recipe, Adaptations)) -> simple_eyre::Result<Self> {
    let (recipe, adaptations) = recipe;

    Ok(adaptations.iter().fold( recipe,
       |r, a|  
       a.adapt_by_weight(&mut r.clone()).unwrap() ))
  }

  pub fn craft(config: Config) -> simple_eyre::Result<(Self, Adaptations)> {
    match config.flours.total_weight {
      Some(tw) => Self::craft_by_ratio(&tw, config),
      None => Self::craft_by_weight(config),
    }
  }

  pub fn recalc(&mut self) -> &mut Self {

      let mut total_starter_weight = Gram::ZERO;
      let mut salt_weight = Gram::ZERO;
      let mut total_flour_weight = Gram::ZERO;
      let mut total_water_weight = Gram::ZERO;

    for ingredient in &self.ingredients {
      match ingredient {
        Ingredient::Starter(s) => {
          total_starter_weight += s.get_total_weight();
          total_flour_weight += s.get_flour_weight();
          total_water_weight += s.get_water_weight();
        },
        Ingredient::Salt(s) => {
          salt_weight += s.weight; 
        },
        Ingredient::Flour(f) => {
          total_flour_weight += f.derive_total_weight()
        }
        Ingredient::Water(w) => {
          total_water_weight += w.weight;
        },
      } 
    }
      self.salt_weight = salt_weight;
      self.total_flour_weight = total_flour_weight;
      self.total_water_weight = total_water_weight;
      let total_weight_excluding_salt = total_flour_weight + total_water_weight;
      self.total_weight = total_weight_excluding_salt + salt_weight;

      self.salt_percentage = salt_weight.as_ratio_of::<SaltPercentage>(&total_flour_weight);
      self.hydration_percentage = total_water_weight.as_ratio_of(&total_flour_weight);

      self.starter_percentage = total_starter_weight.as_ratio_of(&total_weight_excluding_salt);
      self
  }

  /// To be used when the start weight used was different then the suggested recipe
  /// This will have impact on the hydration, and flour content but the explicitly added
  /// flour/water are assumed not to be touched
  pub fn set_starter_weight(&mut self, weight: Gram, hydration: StarterHydrationPercentage) -> simple_eyre::Result<Self> {
    let mut recipe = self.clone();
    let starter = self
                  .get_starter().unwrap()
                  .reset(weight, hydration);
    replace_element!(recipe.ingredients, Ingredient::Starter(_), Ingredient::Starter(starter));
    Ok(recipe.recalc().to_owned())
  }

  pub fn set_water_weight(&mut self, weight: Gram) -> simple_eyre::Result<Self> {
    let mut recipe = self.clone();
    let water = self
              .get_water().unwrap()
              .reset(weight);
    replace_element!(recipe.ingredients, Ingredient::Water(_), Ingredient::Water(water));
    Ok(recipe.recalc().to_owned())
  }
}



impl std::fmt::Display for Recipe {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "{}\n Total weight: {}\n Flour: {}\n Water({}): {}\n Salt({}): {}\n\n{}{}{}\n   {}", 
            "Recipe".bold().underline(),
            self.total_weight,
            self.total_flour_weight,
            self.hydration_percentage,
            self.total_water_weight,
            self.salt_percentage,
            self.salt_weight,
            "Ingredients: using ".bold().underline(),
            self.starter_percentage.to_string().bold().underline(),
            " starter(percentage of all liquids and flours)".bold().underline(),
            self.ingredients.iter()
                 .map(|i| i.to_string()).collect::<Vec<String>>()
                 .join("\n   ")
        )
  }
}
