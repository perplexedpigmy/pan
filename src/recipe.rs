use sequence;
use lazy_static::lazy_static;
use crate::common::Gram;
use crate::ingredient::{
    salt::Salt,
    water::Water,
    flour::Flour,
    flour::FlourItem,
    starter::Starter,
    ingredient::Ingredient
};

#[derive(Debug)]
pub struct Recipe {
    total_weight: Gram,
    hydration_percentage: f32,
    total_flour_weight: Gram,
    total_water_weight: Gram,
    salt_weight: Gram,
    salt_percentage: f32,
    starter_percentage: f32,

    ingredients: Vec<Ingredient>,
}

lazy_static! {
    static ref DEFAULT_TOTAL_FLOUR_WEIGHT: Gram = 600.0.into(); // In grams
    static ref DEFAULT_HYDRATION_PERCENTAGE: f32 = 0.7;         // 70% hydration
    static ref DEFAULT_SALT_PERCENTAGE: f32 = 0.02;             // 2% salt content
    static ref DEFAULT_STARTER_PERCENTAGE: f32 = 0.1;           // 10% starter of flour weight
    static ref DEFAULT_STARTER_HYDRATION: f32 = 1.0;            // 100% hydration
}

impl Recipe {
    // Flour types
    pub const WHITE_FLOUR: &str = "White";
    pub const RYE_FLOUR: &str = "Rye";
    pub const _SPELT_FLOUR: &str = "Spelt";
    
    pub fn default() -> Recipe {
        let hydration_percentage = *DEFAULT_HYDRATION_PERCENTAGE;
        let salt_percentage = *DEFAULT_SALT_PERCENTAGE;
        let total_flour_weight = *DEFAULT_TOTAL_FLOUR_WEIGHT;
        let total_water_weight = total_flour_weight * hydration_percentage;
        let starter_percentage = *DEFAULT_STARTER_PERCENTAGE;

        let salt_weight = total_flour_weight * salt_percentage;
        let total_weight_excl_salt = total_flour_weight + total_water_weight;
        let total_weight = total_weight_excl_salt + salt_weight;

        let starter = Starter::create(total_weight_excl_salt, *DEFAULT_STARTER_HYDRATION, starter_percentage);
        let flour_weight = total_flour_weight - starter.get_flour_weight();

        let water_weight = total_water_weight - starter.get_water_weight(); 


        let flour = Flour { 
            name: String::from(Recipe::WHITE_FLOUR), 
            weight: flour_weight.into(),
        };
        let water = Water { 
            weight: water_weight.into(), 
        };
        
        let salt = Salt { 
            weight: salt_weight.into(),
        };

        Recipe {
            total_weight,
            salt_weight, 
            salt_percentage, 
            hydration_percentage,
            total_flour_weight,
            total_water_weight,
            starter_percentage,
            ingredients: vec![
                Ingredient::Flour(flour),
                Ingredient::Water(water),
                Ingredient::Salt(salt),
                Ingredient::Starter(starter),
            ]
        }

    }

    pub fn get_starter(&self) -> Option<&Starter> {
        for ingredient in &self.ingredients {
            if let Ingredient::Starter(s) = ingredient {
                return Some(s);
            }
        }
        None
    }

    pub fn set_total_flours(&self, total_flour_weight: Gram, flours: &Vec<FlourItem>) -> Recipe {
        let (starter_hydration, starter_percentage) = match self.get_starter() {
            Some(s) => (s.get_hydration(), self.starter_percentage),
            None => (*DEFAULT_STARTER_HYDRATION, *DEFAULT_STARTER_PERCENTAGE),
        };

        let total_water_weight = total_flour_weight * self.hydration_percentage;
        let salt_weight = total_flour_weight * self.salt_percentage;
        let total_weight_excl_salt = total_flour_weight + total_water_weight;

        let total_weight = total_weight_excl_salt + salt_weight;
        let starter = Starter::create(total_weight_excl_salt, starter_hydration, starter_percentage);
        println!("starter: {:#?} ", starter);
        let starter_flour_weight = starter.get_flour_weight();
        let water_weight = total_water_weight - starter.get_water_weight(); 

        let mut seq = sequence!(starter_flour_weight, 0.0);
        println!("seq: {:?}", seq);
        println!("starter_flour_weight: {:?}", starter_flour_weight);
        let mut ingredients: Vec<Ingredient> = flours.iter().map(|f| Ingredient::Flour(Flour{
            name: f.name.clone(), 
            weight: Gram::from(total_flour_weight * f.percentage - seq.next().unwrap_or(0.0))
        })).collect();

        ingredients.push(Ingredient::Starter(starter));

        ingredients.push(Ingredient::Salt(Salt { 
            weight: salt_weight.into()
        }));

        ingredients.push(Ingredient::Water(Water { 
            weight: water_weight.into()
        }));

        Recipe {
            total_weight,
            salt_weight, 
            salt_percentage: self.salt_percentage, 
            hydration_percentage: self.hydration_percentage,
            total_flour_weight,
            total_water_weight,
            starter_percentage,     
            ingredients,
        }
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Summary:\n Total weight: {}\n Flour: {}\n Water: {}({}%)\n Salt: {}({}%)\n\nIngredients: (using {}% starter)\n   {}", 
            self.total_weight,
            self.total_flour_weight,
            self.total_water_weight,
            self.hydration_percentage * 100.0,
            self.salt_weight,
            self.salt_percentage * 100.0,
            self.starter_percentage * 100.0,
            self.ingredients.iter()
                 .map(|i| i.to_string()).collect::<Vec<String>>()
                 .join("\n   ")
        )
    }
}
