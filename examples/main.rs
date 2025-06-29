extern crate sdl2;

use rand::Rng;
use std::sync::Arc;

use nalgebra::*;

use cellular_rustomata::renderer::Renderer;
use cellular_rustomata::rulesets::Mazectric as Rule;
use cellular_rustomata::{Engine, RetrievalMode};

pub fn main() {
    let mut rng = rand::rng();
    let size = 256;
    let neighborhood = 3;
    let grid = DMatrix::from_fn(size, (size as f32 * (16f32 / 9f32)) as usize, |_, _| {
        rng.random_range(0..=100) / 99
    });
    let rules = Rule::new();
    let engine = Engine::new(
        grid,
        Arc::new(rules),
        (neighborhood, neighborhood),
        RetrievalMode::Wrapping,
    )
    .unwrap();
    let mut renderer = Renderer::new(1920, 1080, engine).unwrap();
    renderer.start_loop(120);
}
