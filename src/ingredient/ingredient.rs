use crate::ingredient::{
    salt::Salt,
    water::Water,
    flour::FlourMix,
    starter::Starter,
};

#[derive(Debug)]
pub enum Ingredient {
   Water(Water), 
   Flour(FlourMix),
   Salt(Salt),
   Starter(Starter),
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Ingredient::Water(w) => w.to_string(),
            Ingredient::Flour(fl) => fl.to_string(),
            Ingredient::Salt(s) => s.to_string(),
            Ingredient::Starter(s) => s.to_string(),
        })
    }
}

