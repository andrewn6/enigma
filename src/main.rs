use yew::prelude::*;
use std::{ops::Deref};
use yew_hooks::use_interval;

#[derive(Clone, Copy)]
struct Vector2 {
  x: f64,
  y: f64
}

#[derive(Clone, Copy)]
struct Projectile {
  position: Vector2,
  velocity: Vector2,
}
  

fn drag_force(v: f64, caliber: f64, ballistic_coefficient: f64) -> f64 {
  let drag_coefficient = 1.0 / (ballistic_coefficient * caliber.powi(2));
  let air_density = 1.225;
  -0.5 * drag_coefficient * air_density * v.powi(2)
}

fn update_velocity(projectile: &mut Projectile, dt: f64, wind: f64, caliber: f64, ballistic_coefficient: f64) {
 let v  =  (projectile.velocity.x.powi(2) + projectile.velocity.y.powi(2)).sqrt();
 let drag = drag_force(v, caliber, ballistic_coefficient);
 let acceleration_x = (wind + drag * projectile.velocity.x / v) / 10.0;
 let acceleration_y = (-9.81 + drag * projectile.velocity.y / v) / 10.0;

 projectile.velocity.x += acceleration_x * dt;
 projectile.velocity.y += acceleration_y * dt;
}

fn update_position(projectile: &mut Projectile, dt: f64) {
  projectile.position.x += projectile.velocity.x * dt;
  projectile.velocity.y += projectile.velocity.y * dt;
}

#[function_component]
fn BallisticCalculator() -> Html {
  let wind = use_state(|| 0.0);
  let elevation = use_state(|| 0.0);
  let caliber = use_state(|| 0.00762);
  let ballistic_coefficient = use_state(|| 0.4);
  let projectile = use_state(|| Projectile {
    position: Vector2 { x: 0.0, y: 0.0 },
    velocity: Vector2 {
            // Decomposing initial velocity into horizontal (x) and vertical (y) components using the elevation angle.
            x: 850.0 * (elevation.deref() * std::f64::consts::PI / 180.0).cos(),
            y: 850.0 * (elevation.deref() * std::f64::consts::PI / 180.0).sin(),
        },
  }); 
  
  let projectile_clone = projectile.clone();

  use_interval(
      {  
        move || {
          let mut projectile_value = *projectile.clone();
          let wind_value = *wind.clone();
          let caliber_value = *caliber.clone();
          let ballistic_coefficient_value = *ballistic_coefficient.clone();
          let dt = 0.01;

          update_velocity(&mut projectile_value, dt, wind_value, caliber_value, ballistic_coefficient_value);
          update_position(&mut projectile_value, dt);

          projectile.set(projectile_value);
        }
      },
      10,
  );

  html! {
    <div>
      <div>{format!("Position: ({}, {})", projectile_clone.position.x, projectile_clone.position.y)}</div>
    </div>
  }


}

fn main() {
  yew::Renderer::<BallisticCalculator>::new().render();
}
