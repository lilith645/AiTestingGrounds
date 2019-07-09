use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::{Rocket, Wall, GoalBox};
use crate::cgmath::Vector2;

pub struct TrainingField {
  rockets: Vec<Rocket>,
  walls: Vec<Wall>,
  goal: GoalBox,
}

impl TrainingField {
  pub fn new(window_size: Vector2<f32>, num_rockets: u32) -> TrainingField {
    let mut rockets = Vec::with_capacity(num_rockets as usize);
    for i in 0..num_rockets {
      rockets.push(Rocket::new(Vector2::new(window_size.x*0.5, 100.0)));
    }
    
    let (walls, goal) = TrainingField::wall_setup_one(window_size);
    
    TrainingField {
      rockets,
      walls,
      goal, 
    }
  }
  
  fn create_wall_outskirt(window_size: Vector2<f32>, thickness: f32) -> Vec<Wall> {
    let mut walls = Vec::with_capacity(4);
    
    //top
    walls.push(Wall::new(Vector2::new(window_size.x*0.5, window_size.y), Vector2::new(window_size.x, thickness)));
    //bottom
    walls.push(Wall::new(Vector2::new(window_size.x*0.5, 0.0), Vector2::new(window_size.x, thickness)));
    //left
    walls.push(Wall::new(Vector2::new(0.0, window_size.y*0.5), Vector2::new(thickness, window_size.y)));
    //right
    walls.push(Wall::new(Vector2::new(window_size.x, window_size.y*0.5), Vector2::new(thickness, window_size.y)));
    
    walls
  }
  
  fn wall_setup_one(window_size: Vector2<f32>) -> (Vec<Wall>, GoalBox) {
    let thickness = 10.0;
    
    let mut walls = TrainingField::create_wall_outskirt(window_size, thickness);
    
    let goal = GoalBox::new(Vector2::new(window_size.x*0.45, window_size.y*0.65));
    
    walls.push(Wall::new(window_size*0.5, Vector2::new(window_size.x*0.3, thickness)));
    
    walls.push(Wall::new(Vector2::new(window_size.x*0.35, window_size.y*0.6), Vector2::new(thickness, window_size.y*0.25)));
    
    (walls, goal)
  }
  
  pub fn update(&mut self, delta_time: f32) {
    for rocket in &mut self.rockets {
      rocket.update(delta_time);
      let rock_pos = rocket.position();
      let rock_size = rocket.size();
      let rock_box = rock_pos.extend(rock_size.x).extend(rock_size.y);
      for wall in &self.walls {
        let wall_pos = wall.position();
        let wall_size = wall.size();
        let wall_box = wall_pos.extend(wall_size.x).extend(wall_size.y);
        if math::box_collision(rock_box, wall_box) {
          rocket.died();
        }
      }
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for rocket in &self.rockets {
      rocket.draw(draw_calls);
    }
    
    for wall in &self.walls {
      wall.draw(draw_calls);
    }
    
    self.goal.draw(draw_calls);
  }
}
