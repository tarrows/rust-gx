use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::convert::TryInto;
use std::time::Duration;

pub struct Game {
  // SDL2 context items
  // context: sdl2::Sdl,
  // window: sdl2::video::Window,
  timer: sdl2::TimerSubsystem, // TODO: Consider to use time crate or else as recommended in TimerSubsystem
  events: sdl2::EventPump,
  canvas: sdl2::render::Canvas<sdl2::video::Window>,
  // Global state
  ticks_count: u32,
  is_running: bool,

  window_width: u32,
  window_height: u32,

  // Each object's state
  dir_paddle: PaddleDirection,
  pos_paddle: Vector2,
  pos_ball: Vector2,
  vel_ball: Vector2, // Velocity
}

#[derive(PartialEq, Clone, Copy)]
enum PaddleDirection {
  Stop = 0,
  Up = -1,
  Down = 1,
}

pub struct Config {
  pub width: u32,
  pub height: u32,
}

struct Vector2 {
  x: f64,
  y: f64,
}

const TITLE: &str = "Pong";
const THICKNESS: u32 = 15;
const PADDLE_HEIGHT: f64 = 100.0;

impl Game {
  pub fn init(config: Config) -> Result<Game, String> {
    let context = sdl2::init()?;
    let timer = context.timer()?;
    let events = context.event_pump()?;
    let video = context.video()?;
    let window = video
      .window(TITLE, config.width, config.height)
      .position_centered()
      .build()
      .unwrap(); // TODO: use std::error::Error

    let canvas = window.into_canvas().build().unwrap(); // TODO: use std::error::Error

    let pos_paddle = Vector2 {
      x: 10.0,
      y: config.height as f64 / 2.0,
    };
    let pos_ball = Vector2 {
      x: config.width as f64 / 2.0,
      y: config.height as f64 / 2.0,
    };
    let vel_ball = Vector2 {
      x: -200.0,
      y: 235.0,
    };

    println!("complete initialize...");

    let game = Game {
      // context: context,
      // window: window,
      timer: timer,
      events: events,
      canvas: canvas,
      ticks_count: 0,
      is_running: true,

      window_width: config.width,
      window_height: config.height,
      dir_paddle: PaddleDirection::Stop,
      pos_paddle: pos_paddle,
      pos_ball: pos_ball,
      vel_ball: vel_ball,
    };

    Ok(game)
  }

  pub fn run_loop(&mut self) {
    //    let mut event_pump = self.context.event_pump().unwrap();

    while self.is_running {
      self.dir_paddle = PaddleDirection::Stop;
      // for event in event_pump.poll_iter() {
      self.process_input();
      // }
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
    // let mut event_pump = self.context.event_pump().unwrap();

    // TODO: Move loop to "run_loop"
    // 'running: loop {
    for event in self.events.poll_iter() {
      // self.dir_paddle = PaddleDirection::Stop;

      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          self.is_running = false;
          // break 'running;
        }
        Event::KeyDown {
          keycode: Some(Keycode::W),
          ..
        } => {
          // println!("Up");
          self.dir_paddle = PaddleDirection::Up;
          // break 'running;
        }
        Event::KeyDown {
          keycode: Some(Keycode::S),
          ..
        } => {
          // println!("Down");
          self.dir_paddle = PaddleDirection::Down;
          // break 'running;
        }
        _ => {} // break 'running,
      }
    }
    // }
  }

  fn update(&mut self) {
    // SDL_TICKS_PASSED seems not exists on rust-sdl2
    // substitute https://github.com/emscripten-ports/SDL2/blob/master/include/SDL_timer.h
    while self.ticks_count + 16 > self.timer.ticks() {
      // println!("{} < {}", self.ticks_count, self.timer.ticks());
      ::std::thread::sleep(Duration::new(0, 1_000_000u32)); // wait 1ms
    }
    // self.ticks_count + 16 - self.timer.ticks() <= 0 -> panicked at 'attempt to subtract with overflow'
    // self.ticks_count.saturating_add(16).saturating_sub(self.timer.ticks()) <= 0 -> never breaks

    let current = self.timer.ticks();
    let delta_time = (current - self.ticks_count) as f64 / 1000.0;
    let delta_time = f64::min(delta_time, 0.05);
    // println!("delta: {}", delta_time);
    self.ticks_count = current;

    // TODO: Remove this if statement (maybe unnecessary)
    if self.dir_paddle != PaddleDirection::Stop {
      let paddle_speed = 300.0;
      let paddle_delta = (self.dir_paddle as i32 as f64) * paddle_speed * delta_time;
      self.pos_paddle.y += paddle_delta;
      // println!("paddleY: {} ({})", self.pos_paddle.y, paddle_delta);

      // Make sure paddle does not move off screen!
      // TODO: move these bounds to game object's field
      let lower_bound = PADDLE_HEIGHT / 2.0 + THICKNESS as f64;
      let upper_bound = self.window_height as f64 - PADDLE_HEIGHT / 2.0 - THICKNESS as f64;

      // println!("(bounded)paddleY: {} < {} < {}", lower_bound, self.pos_paddle.y, upper_bound);

      // ref. https://doc.rust-lang.org/std/primitive.f64.html#method.clamp
      self.pos_paddle.y = self.pos_paddle.y.min(upper_bound).max(lower_bound);
    }

    // Update Ball
    self.pos_ball.x += self.vel_ball.x * delta_time;
    self.pos_ball.y += self.vel_ball.y * delta_time;

    // If the ball go off the screen, end the game
    if self.pos_ball.x <= 0.0 {
      self.is_running = false;
    }

    // Bounce the Ball
    let diff = (self.pos_paddle.y - self.pos_ball.y).abs();
    let close_enough = diff <= PADDLE_HEIGHT / 2.0;
    let collect_x_position = self.pos_ball.x <= 25.0 && 20.0 <= self.pos_ball.x;
    let moving_to_left = self.vel_ball.x < 0.0;

    if close_enough && collect_x_position && moving_to_left {
      self.vel_ball.x *= -1.0
    }

    // Did the ball collide with the right wall?
    if self.pos_ball.x >= 1024.0 - THICKNESS as f64 && self.vel_ball.x > 0.0 {
      self.vel_ball.x *= -1.0
    }

    // Did the ball collide with the top wall?
    if self.pos_ball.y <= THICKNESS as f64 && self.vel_ball.y < 0.0 {
      self.vel_ball.y *= -1.0
    }

    // Did the ball collide with the bottom wall?
    if self.pos_ball.y >= (768 - THICKNESS).into() && self.vel_ball.y > 0.0 {
      self.vel_ball.y *= -1.0
    }
  }

  fn generate_output(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 255));
    self.canvas.clear();

    // draw walls
    self.canvas.set_draw_color(Color::RGB(0, 255, 0));

    let bottom_wall_y = (self.window_height - THICKNESS).try_into().unwrap();
    let right_wall_x = (self.window_width - THICKNESS).try_into().unwrap();

    let top_wall = Rect::new(0, 0, self.window_width, THICKNESS);
    let bottom_wall = Rect::new(0, bottom_wall_y, self.window_width, THICKNESS);
    let right_wall = Rect::new(right_wall_x, 0, THICKNESS, self.window_height);
    let walls = [top_wall, bottom_wall, right_wall];

    self.canvas.fill_rects(&walls).unwrap();

    // draw paddle
    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    let paddle_x = (self.pos_paddle.x - THICKNESS as f64 / 2.0).trunc() as i32;
    let paddle_y = (self.pos_paddle.y - PADDLE_HEIGHT / 2.0).trunc() as i32;
    let paddle = Rect::new(paddle_x, paddle_y, THICKNESS, PADDLE_HEIGHT as u32);
    // self.paddle.reposition((paddle_x, paddle_y));
    self.canvas.fill_rect(paddle).unwrap(); // TODO: switch to Result<T, E>

    let ball_x = (self.pos_ball.x - THICKNESS as f64 / 2.0).trunc() as i32;
    let ball_y = (self.pos_ball.y - THICKNESS as f64 / 2.0).trunc() as i32;
    let ball = Rect::new(ball_x, ball_y, THICKNESS, THICKNESS);
    // self.ball.reposition((ball_x, ball_y));
    self.canvas.fill_rect(ball).unwrap(); // TODO: switch to Result<T, E>

    self.canvas.present();
  }
}
