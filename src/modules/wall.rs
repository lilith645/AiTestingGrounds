use maat_graphics::DrawCall;

use crate::cgmath::{Vector2, Vector4};

pub struct Wall {
  position: Vector2<f32>,
  size: Vector2<f32>,
}

impl Wall {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Wall {
    Wall {
      position,
      size,
    }
  }
  
  pub fn position(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_coloured(self.position, self.size, Vector4::new(0.5, 0.0, 0.0, 1.0), 0.0));
  }
}
