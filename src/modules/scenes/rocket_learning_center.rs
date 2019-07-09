use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use hlua::Lua;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::cgmath::{Vector2, Vector4};

use crate::modules::TrainingField;

pub struct RocketLearningCenter {
  data: SceneData,
  training_field: TrainingField,
}

impl RocketLearningCenter {
  pub fn new(window_size: Vector2<f32>) -> RocketLearningCenter {
    RocketLearningCenter {
      data: SceneData::new(window_size, Vec::new()),
      training_field: TrainingField::new(window_size, 10),
    }
  }
}

impl Scene for RocketLearningCenter {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(RocketLearningCenter::new(window_size))
  }
  
  fn update(&mut self, _ui: Option<&Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    self.training_field.update(delta_time);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::draw_coloured(dim*0.5, dim, Vector4::new(0.2, 0.2, 0.2, 1.0), 0.0));
    
    self.training_field.draw(draw_calls);
  }
}
