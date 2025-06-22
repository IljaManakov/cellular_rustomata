use crate::{CellStateType, Engine};
use colorous::{Gradient, INFERNO};
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default)]
struct FPSCounter {
    pub frame_count: u32,
    last_time: Option<Instant>,
}

impl FPSCounter {
    pub fn call(&mut self, window: &mut Window) {
        self.frame_count += 1;
        match self.last_time {
            Some(last_time) => {
                if last_time.elapsed() > Duration::from_secs(1) {
                    window.set_title(&format!("FPS: {}", self.frame_count)).unwrap();
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

struct ColorMap {
    colors: Gradient,
    map: RefCell<HashMap<CellStateType, Color>>,
}

impl ColorMap {
    pub fn new(gradient: Gradient) -> Self {
        ColorMap {
            colors: gradient,
            map: RefCell::new(HashMap::new()),
        }
    }

    pub fn to_color(&self, value: CellStateType) -> Color {
        self.map
            .borrow_mut()
            .entry(value)
            .or_insert_with(|| {
                let (r, g, b) = self.colors.eval_rational(value as usize, 2).as_tuple();
                Color::RGB(r, g, b)
            })
            .clone()
    }
}

pub struct Renderer {
    window: Window,
    canvas: Canvas<Window>,
    events: EventPump,
    cell_engine: Engine,
    fps: Option<FPSCounter>,
    color_map: ColorMap,
}

impl Renderer {
    pub fn new(width: u32, height: u32, cell_engine: Engine) -> Result<Renderer, String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()?
            .window("cellular rustomata", width, height)
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.clone().into_canvas().build().map_err(|e| e.to_string())?;
        let events = sdl_context.event_pump()?;

        Ok(Renderer {
            window,
            canvas,
            events,
            cell_engine,
            fps: None,
            color_map: ColorMap::new(INFERNO),
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
                self.canvas.set_draw_color(self.color_map.to_color(value).clone());
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
        if let Some(fps) = &mut self.fps {
            fps.call(&mut self.window)
        };
    }

    pub fn start_loop(&mut self, fps: u32) {
        self.cell_engine.paused = true;
        'main: loop {
            self.cell_engine.step();
            self.draw();
            self.show_fps();
            if self.handle_events().is_some() {
                break 'main;
            };
            std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / fps));
        }
    }
}
