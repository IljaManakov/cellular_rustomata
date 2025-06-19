extern crate sdl2;

use rand::Rng;

use nalgebra::*;

use cellular_rustomata::renderer::Renderer;
use cellular_rustomata::{CellStateType, Engine, Neighborhood, RetrievalMode};

fn test_neighborhood() {
    print!("\x1b[2J\x1b[?25l");
    let grid = DMatrix::from_row_slice(
        5,
        5,
        &[
            1, 2, 3, 0, 10, 4, 5, 6, 0, 10, 7, 8, 9, 0, 10, 1, 2, 3, 0, 10, 7, 8, 9, 0, 10,
        ],
    );
    // let engine = Engine::new(grid.into(), vec![], [3, 3], RetrievalMode::Wrapping).unwrap();
    // for (index, updated) in (0..10).zip(engine) {
    //     print!("\x1b[;H");
    //         println!("{}",updated);
    //         thread::sleep(time::Duration::from_millis(500));
    // }
    // print!("\x1b[;H");
    // println!("{}", engine.get_neighbourhood(&[0, 0]));
    // print!("\x1b[;H");
    // println!("{}", engine.get_neighbourhood(&[0, 1]));
    // print!("\x1b[;H");
    // println!("{}", engine.get_neighbourhood(&[2, 2]));
}

pub fn main() {
    let mut rng = rand::rng();
    let size = 64;
    let grid = DMatrix::from_fn(size, size, |_, _| rng.random_range(0..=1));
    let rules = vec![
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s < 2 {
                Some(0)
            } else {
                None
            }
        },
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s == 3 {
                Some(1)
            } else {
                None
            }
        },
        |n: &Neighborhood, s: &CellStateType| {
            if n.iter().sum::<CellStateType>() - s > 3 {
                Some(0)
            } else {
                None
            }
        },
    ];
    let engine = Engine::new(grid, rules, [3, 3], RetrievalMode::Wrapping).unwrap();
    let mut renderer = Renderer::new(640, 640, engine).unwrap();
    renderer.start_loop(60);
}
