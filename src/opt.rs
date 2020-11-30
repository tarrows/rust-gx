use structopt::clap::arg_enum;
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "game-programming-in-rust")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
  #[structopt(
    short = "g",
    long = "game",
    default_value("pong"),
    possible_values(&GameTitle::variants()),
    case_insensitive = true
  )]
  pub title: GameTitle,
}

arg_enum! {
  #[derive(Debug)]
  pub enum GameTitle {
    Pong,
  }
}
