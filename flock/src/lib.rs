mod vector2d;
mod utils;

extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use vector2d::Vector2D;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

/**
 * console.log
 */
extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


/**
 * Boid
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub acceleration: Vector2D,
    pub color: u32,
}

/**
 * BoidLite for use in wasm
 */
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct BoidLite {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub color: u32,
}   

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    search_radius: f64,
    boids: Vec<Boid>,
    alignment_weight: f64,
    cohesion_weight: f64,
    separation_weight: f64,
    max_force: f64,
    max_speed: f64,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn new(height: u32, width: u32, num_boids: u32, search_radius: f64, alignment_weight: f64, cohesion_weight: f64, separation_weight: f64, max_force: f64, max_speed: f64) -> Universe {
        utils::set_panic_hook();

        let mut boids = Vec::new();
        for _ in 0..num_boids {
            let x = js_sys::Math::random() * width as f64;
            let y = js_sys::Math::random() * height as f64;
            let boid = Boid::new(x, y,js_sys::Math::random() * 2.0 - 1.0, js_sys::Math::random() * 2.0 - 1.0, (js_sys::Math::random() * 0xFFFFFF as f64) as u32);
            boids.push(boid);
        }
        Universe {
            width,
            height,
            search_radius,
            boids,
            alignment_weight,
            cohesion_weight,
            separation_weight,
            max_force,
            max_speed,
        }
    }
    
    pub fn set_search_radius(&mut self, search_radius: f64) {
        self.search_radius = search_radius;
    }

    pub fn set_alignment_weight(&mut self, alignment_weight: f64) {
        self.alignment_weight = alignment_weight;
    }

    pub fn set_cohesion_weight(&mut self, cohesion_weight: f64) {
        self.cohesion_weight = cohesion_weight;
    }

    pub fn set_separation_weight(&mut self, separation_weight: f64) {
        self.separation_weight = separation_weight;
    }

    pub fn set_max_force(&mut self, max_force: f64) {
        self.max_force = max_force;
    }

    pub fn set_max_speed(&mut self, max_speed: f64) {
        self.max_speed = max_speed;
    }

    pub fn tick(&mut self) {
        let boids = self.boids.clone();
        self.boids = self
            .boids
            .iter()
            .enumerate()
            .map(|(pos, boid)| {
                let mut current_boid = boid.clone();
                current_boid.update(pos, &boids,  self.search_radius, self.alignment_weight, self.cohesion_weight, self.separation_weight, self.max_force, self.max_speed, self.height as f64, self.width as f64);
                current_boid
            })
            .collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_positions(&self) -> JsValue {
        JsValue::from_serde(&self.boids.iter().map(|v| BoidLite{
            x: v.position.x,
            y: v.position.y,
            vx: v.velocity.x,
            vy: v.velocity.y,
            color: v.color,
        }).collect::<Vec<BoidLite>>()).unwrap()
    }
}

impl Boid {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, color: u32) -> Boid {
        Boid {
            position: Vector2D::new(x, y),
            velocity: Vector2D::new(vx, vy),
            acceleration: Vector2D::new(0.0, 0.0),
            color,
        }
    }

    pub fn update(&mut self, pos: usize, boids: &Vec<Boid>, search_radius: f64, alignment_weight: f64, cohesion_weight: f64, separation_weight: f64, max_force: f64, max_speed: f64, height: f64, width: f64) {
        self.wrap(width, height);
        self.align(pos, boids, search_radius, alignment_weight, max_force);
        self.cohere(pos, boids, search_radius, cohesion_weight, max_force);
        self.separate(pos, boids, search_radius, separation_weight, max_force);
        self.position = self.position.add(&self.velocity);
        self.velocity = self.velocity.add(&self.acceleration);
        self.velocity = self.velocity.limit(max_speed);
        self.acceleration = self.acceleration.scale(0.0); //Reset acceleration
    }

    fn wrap(&mut self, width: f64, height: f64) {
        if self.position.x < 0.0 {
            self.position.x = width;
        } else if self.position.x > width {
            self.position.x = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = height;
        } else if self.position.y > height {
            self.position.y = 0.0;
        }
    }

    fn align(&mut self, pos: usize, boids: &Vec<Boid>, search_radius: f64, alignment_weight: f64, max_force: f64) {
        let mut sum = Vector2D::new(0.0, 0.0);
        let mut count = 0;
        for (i, boid) in boids.iter().enumerate() {
            if i != pos {
                let d = self.position.distance(&boid.position);
                if d > 0.0 && d < search_radius {
                    sum = sum.add(&boid.velocity);
                    count += 1;
                }
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f64);
            sum = sum.limit(max_force);
            sum = sum.scale(alignment_weight);
            self.acceleration = self.acceleration.add(&sum);
        }
    }

    fn cohere(&mut self, pos: usize, boids: &Vec<Boid>, search_radius: f64, cohesion_weight: f64, max_force: f64) {
        let mut sum = Vector2D::new(0.0, 0.0);
        let mut count = 0;
        for (i, boid) in boids.iter().enumerate() {
            if i != pos {
                let d = self.position.distance(&boid.position);
                if d > 0.0 && d < search_radius {
                    sum = sum.add(&boid.position);
                    count += 1;
                }
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f64);
            let mut steer = sum.sub(&self.position);
            steer = steer.limit(max_force);
            steer = steer.scale(cohesion_weight);
            self.acceleration = self.acceleration.add(&steer);
        }
    }

    fn separate(&mut self, pos: usize, boids: &Vec<Boid>, search_radius: f64, separation_weight: f64, max_force: f64) {
        let mut sum = Vector2D::new(0.0, 0.0);
        let mut count = 0;
        for (i, boid) in boids.iter().enumerate() {
            if i != pos {
                let d = self.position.distance(&boid.position);
                if d > 0.0 && d < search_radius {
                    let diff = self.position.sub(&boid.position);
                    let diff = diff.scale(1.0 / d);
                    sum = sum.add(&diff);
                    count += 1;
                }
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f64);
            sum = sum.limit(max_force);
            sum = sum.scale(separation_weight);
            self.acceleration = self.acceleration.add(&sum);
        }
    }

}
