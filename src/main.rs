mod game;
use game::{Config, Game};

fn main() {
  let config = Config {
    width: 1024,
    height: 768,
  };
  let mut game = Game::init(config).unwrap();
  game.run_loop();
  game.shutdown();
}
