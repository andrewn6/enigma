use yew::prelude::*;
use yew::hooks::use_interval;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Vector2 {
  x: f64,
  y: f64
}

struct Projectile {
  position: Vector2,
  velocity: Vector2,
}
  
struct Model {
  caliber: f64
}

fn drag_force(v: f64, caliber: f64, ballistic_coefficient: f64) -> f64 {
  let drag_coefficient = 1.0 / (ballistic_coefficient * caliber.powi(2));
  let air_density = 1.225;
  -0.5 * drag_coefficient * air_density * v.powi(2)
}

fn update_velocity(projectile: &mut Projecttile, dt: f64, wind: f64, caliber: f64, ballistic::coefficient: f64) {
 let v  =  (projectile.velocity.x.powi(2) = projectile.velocity.y.powi(2)).sqrt();
 let drag = drag_force(v, caliber, ballistic_coefficient);
 let acceleration_x = (wind + drag * projectile.velocity.x / v) / 10.0;
 let acceleration_y = (-9.81 + drag * projecttile.velocity.y / v) / 10.0;

 projectile.velocity.x += acceleration_x * dt;
 projectile.velocity.y += acceleration_y * dt;
}

fn update_position(projectile: &mut Projectile, dt: f64) {
  projectile.position.x += projectile.velocity.x * dt;
  projectile.velocity.y += projectile.velocity.y * dt;
}
