use std::borrow::BorrowMut;
use std::{ops::Deref};
use std::rc::Rc;
use std::cell::RefCell;

use nalgebra as na;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew_hooks::use_interval;
use yew::events::SubmitEvent;

use rapier3d::na::Vector3;
use rapier3d::prelude::{self, ImpulseJointSet, MultibodyJointSet, CCDSolver};
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::prelude::{ColliderBuilder, ColliderSet, IntegrationParameters, IslandManager};
use rapier3d::pipeline::PhysicsPipeline;

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

#[derive(Clone)]
struct PhysicsState {
  pipeline: Rc<RefCell<PhysicsPipeline>>,
  rigid_bodies: Rc<RefCell<RigidBodySet>>,
  colliders: Rc<RefCell<ColliderSet>>,
  island_manager: Rc<RefCell<IslandManager>>,
  broad_phase: Rc<RefCell<rapier3d::geometry::BroadPhase>>,
  narrow_phase: Rc<RefCell<rapier3d::geometry::NarrowPhase>>,
}
  
impl PhysicsState {
  fn new() -> Self {
    PhysicsState {
      pipeline: Rc::new(RefCell::new(PhysicsPipeline::new())),
      rigid_bodies: Rc::new(RefCell::new(RigidBodySet::new())),
      colliders: Rc::new(RefCell::new(ColliderSet::new())),
      island_manager: Rc::new(RefCell::new(IslandManager::new())),
      broad_phase: Rc::new(RefCell::new(rapier3d::geometry::BroadPhase::new())),
      narrow_phase: Rc::new(RefCell::new(rapier3d::geometry::NarrowPhase::new())),
    }
  }

  fn step(&mut self, dt: f32) {
    let integration_parameters = IntegrationParameters::default();
    let gravity = Vector3::new(0.0, -9.81, 0.0);
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = (); 
    let event_handler = (); 

    let mut rigid_bodies_borrowed = self.rigid_bodies.borrow_mut();
    let mut colliders_borrowed = self.colliders.borrow_mut();
    let mut island_manager_borrowed = self.island_manager.borrow_mut();
    let mut broad_phase_borrowed = self.broad_phase.borrow_mut();
    let mut narrow_phase_borrowed = self.narrow_phase.borrow_mut();
   
    self.pipeline.borrow_mut().step(
      &gravity,
      &integration_parameters,
      &mut island_manager_borrowed,
      &mut broad_phase_borrowed,
      &mut narrow_phase_borrowed,
      &mut rigid_bodies_borrowed,
      &mut colliders_borrowed,
      &ccd_solver,
      &physics_hooks,
      &event_handler
    );
  }
}

fn drag_force(v: f64, caliber: f64, ballistic_coefficient: f64) -> f64 {
  let drag_coefficient = 1.0 / (ballistic_coefficient * caliber.powi(2));
  let air_density = 1.225;
  -0.5 * drag_coefficient * air_density * v.powi(2)
}

fn update_velocity(projectile: &mut Projectile, dt: f64, wind: f64, caliber: f64, ballistic_coefficient: f64) {
 let v  =  (projectile.velocity.x.powi(2) + projectile.velocity.y.powi(2)).sqrt();
 if v != 0.0 {
    let drag = drag_force(v, caliber, ballistic_coefficient);
    let acceleration_x = (wind + drag * projectile.velocity.x / v);
    let acceleration_y = (-9.81 + drag * projectile.velocity.y / v);

    projectile.velocity.x += acceleration_x * dt;
    projectile.velocity.y += acceleration_y * dt
 }
}

fn update_position(projectile: &mut Projectile, dt: f64) {
  projectile.position.x += projectile.velocity.x * dt;
  projectile.position.y += projectile.velocity.y * dt;
}

#[function_component]
fn BallisticCalculator() -> Html {
  let wind = use_state(|| 0.0);
  let elevation = use_state(|| 0.0);
  let caliber = use_state(|| 0.00762);
  let ballistic_coefficient = use_state(|| 0.4);
  let projectile = use_state(|| Projectile {
    position: Vector2 { x: 0.0, y: 0.0 },
    velocity: Vector2 { x: 0.0, y: 0.0 },
  });

  let physics = use_state(|| Rc::new(RefCell::new(PhysicsState::new())));

  let gravity = Vector3::new(0.0, -9.81, 0.0);

  let on_wind_input = {
      let wind = wind.clone();
      Callback::from(move |e: InputEvent| {
          if let Some(input) = e.target().unwrap().dyn_ref::<HtmlInputElement>() {
              if let Ok(value) = input.value().parse() {
                  wind.set(value);
              }
          }
      })
  };

  let on_elevation_input =  {
    let elevation = elevation.clone();
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target().unwrap().dyn_ref::<HtmlInputElement>() {
            if let Ok(value) = input.value().parse() {
                elevation.set(value);
            }
        }
    })
  };

  let on_caliber_input = {
    let caliber = caliber.clone();
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target().unwrap().dyn_ref::<HtmlInputElement>() {
            if let Ok(value) = input.value().parse() {
                caliber.set(value);
            }
        }
    })
  };
    
  let on_ballistic_coefficient_input = {
    let ballistic_coefficient = ballistic_coefficient.clone();
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target().unwrap().dyn_ref::<HtmlInputElement>() {
            if let Ok(value) = input.value().parse() {
                ballistic_coefficient.set(value);
            }
        }
    })
  };

  let on_submit = Callback::from({
    let physics = physics.clone();
    let elevation = elevation.clone();
    let projectile = projectile.clone();

    move |e: SubmitEvent| {
      e.prevent_default();
      let new_velocity = Vector2 {
        x: 850.0 * (*elevation.deref() * std::f64::consts::PI / 180.0).cos(),
        y: 850.0 * (*elevation.deref() * std::f64::consts::PI / 180.0).sin(),
      };
      let mut proj = *projectile.deref();
      proj.velocity = new_velocity;
      projectile.set(proj);

      let lin_velocity = Vector3::new(new_velocity.x as f32, new_velocity.y as f32, 0.0);

      let bullet = RigidBodyBuilder::dynamic()
        .translation(Vector3::new(0.0, 0.0, 0.0))
        .linvel(lin_velocity)
        .build();

      let bullet_handle = physics.set(bullet);

      let collider = ColliderBuilder::ball(0.00762).build();
      let collider_handle = physics.insert(collider);
      physics[collider_handle].parent();
    }
  });

  let projectile_clone = projectile.clone();
  let projectile_clone_for_position = projectile.clone();

  use_interval(
      move || {
          let mut projectile_value = *projectile_clone.deref();;
          let wind_value = *wind.deref();
          let caliber_value = *caliber.deref();
          let ballistic_coefficient_value = *ballistic_coefficient.deref();
          let dt = 0.01;

          update_velocity(&mut projectile_value, dt, wind_value, caliber_value, ballistic_coefficient_value);
          update_position(&mut projectile_value, dt);

          projectile.set(projectile_value);
      },
      10,
  );
  
  html! {
    <div>
    <form onsubmit={on_submit}>
      <input type="number" step="0.01" placeholder="Wind" oninput={on_wind_input} />
      <input type="number" placeholder="Elevation" oninput={on_elevation_input} />
      <input type="number" step="0.00001" placeholder="Caliber" oninput={on_caliber_input} />
      <input type="number" placeholder="Ballistic Coefficient" oninput={on_ballistic_coefficient_input} step="0.01" min="0" max="1" />
      <button type="submit">{"Submit"}</button>
    </form>
    <div>{format!("Position: ({}, {})", projectile_clone_for_position.position.x, projectile_clone_for_position.position.y)}</div>
    </div>
  }

}

fn main() {
  yew::Renderer::<BallisticCalculator>::new().render();
}