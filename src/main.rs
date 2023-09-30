use pn::recipe::Recipe;
use pn::ingredient::flour::FlourItem;

fn main() {

    let args = pn::get_args().unwrap();
    let bread_recipe = Recipe::default()
                .set_total_flours(args.weight.into(), 
                                  &args.flours,
    );
    println!("{:#}", bread_recipe);
}
