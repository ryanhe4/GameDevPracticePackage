use crate::glfw::*;
use crate::glu_sys::*;
use crate::Game2D::*;

pub struct CustomProgram {
  body: Game2D,
}

pub trait Custom {
  fn update(&mut self) {}
  fn run(&mut self);
}

impl Custom for CustomProgram {
  fn update(&mut self) {
    println!("this is Wrapper's update!");
  }
  fn run(&mut self) {
    while !self.body.glfw_window.should_close() {
      // pre draw
      self.body.glfw_window.make_current();
      unsafe {
        glClearColor(1.0, 1.0, 1.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_MODELVIEW);
        glPushMatrix();

        self.body.draw_grid();
        self.update();
        glPopMatrix();
      }

      // post draw
      self.body.glfw_window.swap_buffers();
      self.body.engine.poll_events();
      for (_, event) in glfw::flush_messages(&self.body.event) {
        println!("{:?}", event);
        match event {
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            self.body.glfw_window.set_should_close(true)
          }
          _ => {}
        }
      }
    }
  }
}

impl CustomProgram {
  pub fn new() -> CustomProgram {
    CustomProgram {
      body: Game2D::new(String::from("asd"), 1280, 720, true),
    }
  }
}
