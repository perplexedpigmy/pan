use pan::recipe::Recipe;
use pan::ingredient::flour::FlourItem;

fn main() {

    let bread = Recipe::default()
                .set_total_flours(1000.into(), 
                                  &vec![
                                    FlourItem::new(Recipe::WHITE_FLOUR, 80.into()), 
                                    FlourItem::new(Recipe::RYE_FLOUR,   20.into()) 
            ])
    ;
    println!("{:#}", bread);
}
