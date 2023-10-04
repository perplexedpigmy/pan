use pn::recipe::Recipe;

fn main() {

    let config = pn::get_args().unwrap();
    let bread_recipe = Recipe::craft(config);
    println!("{:#}", bread_recipe);
    
    // println!("{:#?}", args.flours);
    // let bread_recipe = Recipe::default()
    //             .set_total_flours(args.weight.into(), 
    //                               &args.flours,
    // );
    // println!("{:#}", bread_recipe);
}
