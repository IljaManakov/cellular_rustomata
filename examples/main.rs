// use std::{thread, time};
use nalgebra::*;

mod lib;

fn main() {
    // print!("\x1b[2J\x1b[?25l");
    let grid = DMatrix::from_row_slice(5, 5,
                                       &[
                                           1., 2., 3., 0., 10.,
                                           4., 5., 6., 0., 10.,
                                           7., 8., 9., 0., 10.,
                                           1., 2., 3., 0., 10.,
                                           7., 8., 9., 0., 10.,
                                       ],
    );
    let engine = lib::Engine::new(grid, vec![], [3, 3], lib::RetrievalMode::Wrapping).unwrap();
    // for (index, updated) in (0..10).zip(engine) {
    //     print!("\x1b[;H");
    //         println!("{}",updated);
    //         thread::sleep(time::Duration::from_millis(500));
    // }
    println!("{}", engine.get_neighbourhood(&[0, 0]));
    println!("{}", engine.get_neighbourhood(&[0, 1]));
    println!("{}", engine.get_neighbourhood(&[2, 2]));
}
