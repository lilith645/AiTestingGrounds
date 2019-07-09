use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::cgmath::Vector2;

pub struct Rocket {
  position: Vector2<f32>,
  size: Vector2<f32>,
  texture: String,
  angle: f32,
  velocity: f32,
  alive: bool,
}

impl Rocket {
  pub fn new(start_pos: Vector2<f32>) -> Rocket {
    Rocket {
      position: start_pos,
      size: Vector2::new(25.0, 25.0),
      texture: "Rocket".to_string(),
      angle: 90.0,
      velocity: 100.0,
      alive: true,
    }
  }
  
  pub fn position(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn died(&mut self) {
    self.alive = false;
  }
  
  pub fn update(&mut self, delta_time: f32) {
    if !self.alive {
      return
    }
    
    let radian_angle = math::to_radians(self.angle);
    self.position += Vector2::new(radian_angle.cos(), radian_angle.sin())*self.velocity*delta_time;
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if self.alive {
      draw_calls.push(DrawCall::draw_textured(self.position, self.size, self.angle-90.0, self.texture.to_string()));
    } else {
      draw_calls.push(DrawCall::draw_textured(self.position, self.size, self.angle-90.0, "DeadRocket".to_string()));
    }
  }
}
