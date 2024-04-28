use crate::common::Gram;
use crate::ingredient::{
  ingredient::Ingredient, salt::Salt, salt::SaltPercentage, starter::Starter,
  starter::StarterPercentage, starter::StarterHydrationPercentage, water::HydrationPercentage, water::Water,
};
use crate::Config;

#[derive(Debug, Clone)]
struct Adaptaions {
  reset_starter_weight: Option<f32>,
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

  adaptations: Adaptaions,
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

  pub fn craft_by_ratio(total_flour_weight: &Gram, config: Config) -> simple_eyre::Result<Self> {
    let total_water_weight = *total_flour_weight * config.hydration;
    let starter = Starter::create(
      *total_flour_weight + total_water_weight,
      config.starter_hydration,
      config.starter_percentage,
    );
    let salt_weight = *total_flour_weight * config.salt_percentage;
    let total_weight = *total_flour_weight + total_water_weight + salt_weight;

    Ok(Recipe {
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
      adaptations: Adaptaions {
       reset_starter_weight: config.reset_starter_weight, 
      }
    })
  }

  fn craft_by_weight(_config: Config) -> simple_eyre::Result<Self> {
    unimplemented!("Only by ratio is supported!");
  }

  pub fn adapt(recipe: Recipe) -> simple_eyre::Result<Self> {
    let starter_hydration = recipe.get_starter().ok_or::<StarterHydrationPercentage>(100.into()).unwrap().get_hydration();
    match recipe.adaptations.reset_starter_weight {
      Some(new_starter_weight) => recipe.set_starter_weight(new_starter_weight.into(), starter_hydration),
      None => Ok(recipe),
    }      
  }

  pub fn craft(config: Config) -> simple_eyre::Result<Self> {
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
  pub fn set_starter_weight(self, weight: Gram, hydration: StarterHydrationPercentage) -> simple_eyre::Result<Self> {
    let mut recipe = self.clone();
    let starter = self
                  .get_starter().unwrap()
                  .reset(weight, hydration);
    replace_element!(recipe.ingredients, Ingredient::Starter(_), Ingredient::Starter(starter));
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
