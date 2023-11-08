use pn::recipe::Recipe;

fn main() {
  let mut bread = pn::get_args().and_then(Recipe::craft).unwrap();
  let bread = bread.set_starter_weight(50.into(), 100.into()).unwrap();
  println!("{:#}", bread);
}
