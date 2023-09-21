#[macro_use]
mod macros;
mod common;
mod ingredient;
mod recipe;

use ingredient::flour::FlourItem;
use recipe::Recipe;

fn main() {
    let bread = Recipe::default()
                .set_total_flours(800.0, 
                                  &vec![
                                    FlourItem::new(Recipe::WHITE_FLOUR, 0.8), 
                                    FlourItem::new(Recipe::RYE_FLOUR,   0.2) 
            ])
    ;
    println!("{:#}", bread);
}
