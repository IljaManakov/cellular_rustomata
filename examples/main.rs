extern crate sdl2;

use rand::Rng;

use nalgebra::*;

use cellular_rustomata::renderer::Renderer;
use cellular_rustomata::{maze_generation, Engine, RetrievalMode};

pub fn main() {
    let mut rng = rand::rng();
    let size = 128;
    let grid = DMatrix::from_fn(size, (size as f32 * (16f32 / 9f32)) as usize, |_, _| {
        rng.random_range(0..=100) / 95
    });
    // let rules = game_of_life();
    let rules = maze_generation!([3], (1, 5));
    let engine = Engine::new(grid, rules, (3, 3), RetrievalMode::Wrapping).unwrap();
    let mut renderer = Renderer::new(1920, 1080, engine).unwrap();
    renderer.start_loop(120);
}
