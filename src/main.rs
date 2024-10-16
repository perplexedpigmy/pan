use pn::recipe::Recipe;

fn main() {
  let recipe = pn::get_args()
        .and_then(Recipe::craft)
        .and_then(Recipe::adapt)
        .unwrap();

  println!("{:#}", recipe);
}
