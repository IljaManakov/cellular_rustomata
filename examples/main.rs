extern crate sdl2;

use rand::Rng;
use std::{time::Duration};

use nalgebra::*;

use cellular_rustomata::{Engine, RetrievalMode, Neighborhood, CellStateType};
use cellular_rustomata::renderer::Renderer;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::FRect;

fn test_neighborhood() {
    print!("\x1b[2J\x1b[?25l");
    let grid = DMatrix::from_row_slice(5, 5,
                                       &[
                                           1, 2, 3, 0, 10,
                                           4, 5, 6, 0, 10,
                                           7, 8, 9, 0, 10,
                                           1, 2, 3, 0, 10,
                                           7, 8, 9, 0, 10,
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
pub fn sdl_test() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut j: i16 = 0;
    'running: loop {
        j = (j + 1) % i16::MAX;
        let i = f32::cos(j as f32 / 255. * std::f32::consts::PI);
        canvas.set_draw_color(Color::RGB(128 - (i * 127.) as u8, 64, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(128 - (i * 32.) as u8, 128 - (i * 64.) as u8,  128 - (i * 96.) as u8));
        let rect = FRect::new(- (i * 335.), 300. - (i * 235.), 64., 64.);
        canvas.fill_frect(rect).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 100_000_000_u32 / 60));
    }

}

pub fn main(){
    let mut renderer = Renderer::new(640, 640).unwrap();
     // let grid = DMatrix::from_row_slice(5, 5,
     //                                   &[
     //                                       10, 20, 30, 0, 100,
     //                                       40, 50, 60, 0, 100,
     //                                       70, 80, 90, 0, 100,
     //                                       10, 20, 30, 0, 100,
     //                                       70, 80, 90, 0, 100,
     //                                   ],
    // );
    let mut rng = rand::rng();
    let grid = DMatrix::from_fn(128, 128, |_, _| rng.random_range(0..=1));
    let rules = vec![
      |n: &Neighborhood, s: &CellStateType| { if n.iter().sum::<CellStateType>() - s < 2 {Some(0)} else {None}},
      |n: &Neighborhood, s: &CellStateType| { if n.iter().sum::<CellStateType>() - s == 3 {Some(1)} else {None}},
      |n: &Neighborhood, s: &CellStateType| { if n.iter().sum::<CellStateType>() - s > 3 {Some(0)} else {None}},
    ];
    let mut engine = Engine::new(grid, rules, [3, 3], RetrievalMode::Wrapping).unwrap();

    renderer.draw(&engine.grid);

    loop {
        engine.step();
        renderer.draw(&engine.grid);
    }
}
