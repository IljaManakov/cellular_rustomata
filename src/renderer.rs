use colorous::INFERNO;
use nalgebra::{Dyn, OMatrix};
use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;
use crate::CellStateType;


pub struct Renderer {
    canvas: Canvas<Window>,
}


impl Renderer {

    pub fn new(width: u32, height: u32) -> Result<Renderer, String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()?
            .window("cellular rustomata", width, height)
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }
    pub fn draw(&mut self, grid: &OMatrix<CellStateType, Dyn, Dyn>) {
        self.canvas.set_logical_size(grid.ncols() as u32, grid.nrows() as u32).map_err(|err| err.to_string()).unwrap();
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
}
