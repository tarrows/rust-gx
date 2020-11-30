mod opt;
mod pong;
use opt::Opt;
use pong::{Config, Game};
use structopt::StructOpt;

fn main() {
  let opt = Opt::from_args();
  println!("{:#?}", opt);
  let config = Config {
    width: 1024,
    height: 768,
  };
  let mut pong = Game::init(config).unwrap();
  pong.run_loop();
  pong.shutdown();
}
