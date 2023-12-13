mod game;

use std::sync::Arc;

use game::{*};

fn main() {
  // Look into different data types.
  let mut minetest = Box::new(Game::new());
  minetest.enter_main_loop();
}
