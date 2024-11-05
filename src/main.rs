// use pn::recipe::Recipe;

use pn::recipe::Recipe;

fn main() {
  pn::get_args()
    .and_then(Recipe::build)
    .and_then(Recipe::display)
    .unwrap();
}
