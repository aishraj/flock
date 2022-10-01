//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate flock;
use std::assert_eq;

use flock::Boid;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

//wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn boid() {
    let boid = Boid::new(0.0, 0.0, 0.0, 0.0, 0);
    assert_eq!(boid.position.x, 0.0);
}

#[wasm_bindgen_test]
fn it_updates_correctly() {
    let boids = vec![
        Boid::new(0.0, 0.0, -1.0, -1.0, 0),
        Boid::new(1.0, 1.0, 1.0, 1.0, 0)
    ];
    let mut boid = &mut boids[0].clone();
    boid.update(0, &boids, 0.1, 0.1, 0.5, 0.5, 0.1, 3.0, 200.0, 200.0);
    assert_eq!(boid.position.x, -1.0);
    assert_eq!(boid.position.y, -1.0);
}

