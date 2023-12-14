pub mod game;

use game::*;

fn main() {
  Game::new(true).enter_main_loop()
}
