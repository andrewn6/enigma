use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew::events::SubmitEvent;
use yew_hooks::use_interval;

use nalgebra as na;
use std::ops::Deref;
use std::f64::consts::PI;

use parry3d::shape::{Cuboid, Shape};
use parry3d::math::Isometry;

pub struct ParryShape {
  width: f32,
  height: f32,
}

#[derive(Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy)]
struct Projectile {
    position: Vector3,
    velocity: Vector3,
}

fn drag_force(v: f64, caliber: f64, ballistic_coefficient: f64) -> Vector3 {
    let drag_coefficient = 1.0 / (ballistic_coefficient * caliber.powi(2));
    let air_density = 1.225;
    Vector3 {
        x: -0.5 * drag_coefficient * air_density * v.powi(2),
        y: -0.5 * drag_coefficient * air_density * v.powi(2),
        z: -0.5 * drag_coefficient * air_density * v.powi(2),
    }
}

fn update_velocity(
    projectile: &mut Projectile,
    dt: f64,
    wind: Vector3,
    caliber: f64,
    ballistic_coefficient: f64,
) {
    let v = (projectile.velocity.x.powi(2) + projectile.velocity.y.powi(2) + projectile.velocity.z.powi(2)).sqrt();
    if v != 0.0 {
        let drag = drag_force(v, caliber, ballistic_coefficient);
        let acceleration_x = wind.x + drag.x * projectile.velocity.x / v;
        let acceleration_y = wind.y + drag.y * projectile.velocity.y / v;
        let acceleration_z = wind.z + drag.z * projectile.velocity.z / v;

        projectile.velocity.x += acceleration_x * dt;
        projectile.velocity.y += acceleration_y * dt;
        projectile.velocity.z += acceleration_z * dt;
    }
}

fn update_position(projectile: &mut Projectile, dt: f64) {
    projectile.position.x += projectile.velocity.x * dt;
    projectile.position.y += projectile.velocity.y * dt;
    projectile.position.z += projectile.velocity.z * dt;
}

#[function_component]
fn BallisticCalculator() -> Html {
    let wind = use_state(|| 0.0);
    let elevation = use_state(|| 0.0);
    let caliber = use_state(|| 0.00762);
    let ballistic_coefficient = use_state(|| 0.4);
    let projectile = use_state(|| Projectile {
        position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    });

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

    let on_elevation_input = {
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
        let elevation = elevation.clone();
        let projectile = projectile.clone();

        move |e: SubmitEvent| {
            e.prevent_default();
            let new_velocity = Vector3 {
                x: 850.0 * (*elevation.deref() * std::f64::consts::PI / 180.0).cos(),
                y: 850.0 * (*elevation.deref() * std::f64::consts::PI / 180.0).sin(),
                z: 0.0,
            };
            let mut proj = *projectile.deref();
            proj.velocity = new_velocity;
            projectile.set(proj);
        }
    });

    let projectile_clone = projectile.clone();
    let projectile_clone_for_position = projectile.clone();

    use_interval(
        move || {
            let mut projectile_value = *projectile_clone.deref();
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