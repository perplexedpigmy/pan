use pn::recipe::Recipe;

fn main() {
  let bread = pn::get_args().and_then(Recipe::craft).unwrap();
  println!("{:#}", bread);
}
