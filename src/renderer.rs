use crate::Engine;
use colorous::INFERNO;
use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct FPSCounter {
    frame_count: u32,
    last_time: Option<Instant>,
}

impl FPSCounter {
    pub fn call(&mut self) {
        self.frame_count += 1;
        match self.last_time {
            Some(last_time) => {
                if last_time.elapsed() > Duration::from_secs(1) {
                    println!("FPS: {}", self.frame_count);
                    self.frame_count = 0;
                    self.last_time = Some(Instant::now());
                }
            }
            None => {
                self.last_time = Some(Instant::now());
            }
        }
    }
}

pub struct Renderer {
    canvas: Canvas<Window>,
    events: EventPump,
    cell_engine: Engine,
    fps: Option<FPSCounter>,
}

impl Renderer {
    pub fn new(width: u32, height: u32, cell_engine: Engine) -> Result<Renderer, String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()?
            .window("cellular rustomata", width, height)
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let events = sdl_context.event_pump()?;

        Ok(Renderer {
            canvas,
            events,
            cell_engine,
            fps: None,
        })
    }
    fn draw(&mut self) {
        let grid = &self.cell_engine.grid;
        self.canvas
            .set_logical_size(grid.ncols() as u32, grid.nrows() as u32)
            .map_err(|err| err.to_string())
            .unwrap();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for i in 0..grid.nrows() {
            for j in 0..grid.ncols() {
                let value = grid[(i, j)];
                let (r, g, b) = INFERNO.eval_rational(value as usize, 2).as_tuple();
                self.canvas.set_draw_color(Color::RGB(r, g, b));
                self.canvas.draw_point(Point::new(j as i32, i as i32)).unwrap();
            }
        }
        self.canvas.present();
    }

    fn handle_events(&mut self) -> Option<bool> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => return Some(true),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self.cell_engine.paused = !self.cell_engine.paused;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    if self.fps.is_some() {
                        self.fps = None
                    } else {
                        self.fps = Some(FPSCounter::default())
                    };
                }
                _ => {}
            }
        }
        None
    }

    fn show_fps(&mut self) {
        if self.fps.is_none() {
            return;
        };
    }

    pub fn start_loop(&mut self, fps: u32) {
        println!("Starting loop");
        self.cell_engine.paused = true;
        'main: loop {
            self.cell_engine.step();
            self.draw();
            if let Some(fps) = &mut self.fps {
                fps.call()
            };
            if self.handle_events().is_some() {
                break 'main;
            };
            std::thread::sleep(Duration::new(0, 100_000_000_u32 / fps));
        }
    }
}
