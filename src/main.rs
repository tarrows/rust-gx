mod game;
use game::{Config, Game};

fn main() {
  let config = Config {
    width: 640,
    height: 480,
  };
  let mut game = Game::init(config).unwrap();
  game.run_loop();
  game.shutdown();
}
