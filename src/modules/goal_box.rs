use maat_graphics::DrawCall;

use crate::cgmath::{Vector2, Vector4};

pub struct GoalBox {
  position: Vector2<f32>,
  size: Vector2<f32>,
}

impl GoalBox {
  pub fn new(position: Vector2<f32>) -> GoalBox {
    GoalBox {
      position,
      size: Vector2::new(40.0, 40.0),
    }
  }
  
  pub fn position(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_coloured(self.position, self.size, Vector4::new(0.0, 0.7, 0.1, 1.0), 0.0));
  }
}
