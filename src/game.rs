use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Game {
  context: sdl2::Sdl,
  // window: sdl2::video::Window,
  canvas: sdl2::render::Canvas<sdl2::video::Window>,
  // ticks_count: u32,
  is_running: bool,
}

pub struct Config {
  pub width: u32,
  pub height: u32,
}

const TITLE: &str = "SDL";

impl Game {
  pub fn init(config: Config) -> Result<Game, String> {
    let context = sdl2::init()?;
    let video = context.video()?;
    let window = video
      .window(TITLE, config.width, config.height)
      .position_centered()
      .build()
      .unwrap(); // TODO: use std::error::Error

    let mut canvas = window.into_canvas().build().unwrap(); // TODO: use std::error::Error

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    println!("complete initialize...");

    let game = Game {
      context: context,
      // window: window,
      canvas: canvas,
      // ticks_count: 0,
      is_running: true,
    };

    Ok(game)
  }

  pub fn run_loop(&mut self) {
    while self.is_running {
      self.process_input();
      self.update();
      self.generate_output();

      // 60fps
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
  }

  pub fn shutdown(&self) {
    println!("shutdown...")
  }

  fn process_input(&mut self) {
    let mut event_pump = self.context.event_pump().unwrap();

    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => {
            self.is_running = false;
            break 'running;
          }
          _ => {}
        }
      }
    }
  }

  fn update(&self) {}

  fn generate_output(&mut self) {
    self.canvas.present();
  }
}
