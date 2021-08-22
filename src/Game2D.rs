use glfw::{Action, Context, Key};

use crate::DrawFunctions::*;

pub struct Game2D {
  width: u32,
  height: u32,
  pub engine: glfw::Glfw,
  pub glfw_window: glfw::Window,
  spf: f32,
  pub draw_grid: bool,
  pub event: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Game2D {
  pub fn new(_title: String, _width: u32, _height: u32, use_full_screen: bool) -> Game2D {
    let game = Game2D::init(_title, _width, _height, use_full_screen);
    game
  }
  fn init(_title: String, _width: u32, _height: u32, use_full_screen: bool) -> Game2D {
    let gl = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = gl
      .create_window(_width, _height, _title.as_str(), glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    Game2D {
      width: _width,
      height: _height,
      engine: gl,
      glfw_window: window,
      spf: 0.10 / 60.0,
      draw_grid: use_full_screen,
      event: events,
    }
  }

  pub fn getTimeStep(&self) -> f32 {
    self.spf
  }

  fn isKeyPressendAndReleased(key: u32) {}
  pub fn draw_grid(&self) {
    if self.draw_grid {
      setLineWidth(1);
      drawGrid(0.5);
    }
  }
}
