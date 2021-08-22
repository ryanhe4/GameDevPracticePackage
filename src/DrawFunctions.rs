use gl;
use glu_sys::*;

pub fn setLineWidth(width: u32) {
  unsafe {
    glLineWidth(width as f32);
  }
}
pub fn drawLine(color0: [f32; 3], position0: [f32; 2], color1: [f32; 3], position1: [f32; 2]) {
  unsafe {
    glBegin(GL_LINES);
    {
      glColor3fv(&color0[0]);
      glVertex2fv(&position0[0]);
      glColor3fv(&color1[0]);
      glVertex2fv(&position1[0]);
    }
    glEnd()
  }
}
pub fn drawGrid(dx: f32) {
  let maxl: f32 = 2.0;
  let dy = dx;

  let mut x: f32 = 0.0;
  while (x < maxl) {
    drawLine([0.0, 0.0, 0.0], [x, -maxl], [0.0, 0.0, 0.0], [x, maxl]);
    x += dx;
  }
  x = -dx;
  while (x > -maxl) {
    drawLine([0.0, 0.0, 0.0], [x, -maxl], [0.0, 0.0, 0.0], [x, maxl]);
    x -= dx;
  }
  let mut y: f32 = 0.0;
  while (y < maxl) {
    drawLine([0.0, 0.0, 0.0], [-maxl, y], [0.0, 0.0, 0.0], [maxl, y]);
    y += dy;
  }
  y = -dy;
  while (y > -maxl) {
    drawLine([0.0, 0.0, 0.0], [-maxl, y], [0.0, 0.0, 0.0], [maxl, y]);
    y -= dy;
  }
}
