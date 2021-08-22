extern crate gl;
extern crate glfw;
extern crate glu_sys;

mod CumstomProgram;
mod DrawFunctions;
mod Game2D;
use CumstomProgram::Custom;
use CumstomProgram::CustomProgram as CP;

fn main() {
    // let mut engine = engine::Engine::new(800, 600);
    // engine.run();

    // Engine2D::new(String::from("text"), 1280, 720, true).run();
    CP::new().run();
}
