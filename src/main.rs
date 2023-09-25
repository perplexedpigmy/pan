use pan::recipe::Recipe;
use pan::ingredient::flour::FlourItem;

fn main() {
    let bread = Recipe::default()
                .set_total_flours(800.into(), 
                                  &vec![
                                    FlourItem::new(Recipe::WHITE_FLOUR, 0.8), 
                                    FlourItem::new(Recipe::RYE_FLOUR,   0.2) 
            ])
    ;
    println!("{:#}", bread);
}
