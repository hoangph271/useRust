use crate::lib::constants::{Direction, HEIGHT, PIXEL_SIZE, WIDTH};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct Snake {
    pub x: i32,
    pub y: i32,
    pub heading: Direction,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.x as f64, self.y as f64, PIXEL_SIZE);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(green, square, transform, gl);
        });
    }

    pub fn update(&mut self) {
        // TODO: Wall...?
        match self.heading {
            Direction::Up => self.y -= 1, // FIXME: Broke here, add wall check
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1, // FIXME: Broke here, add wall check
            Direction::Right => self.x += 1,
            Direction::StandBy => {}
        }
    }

    pub fn is_alive(&self) -> bool {
        if self.x < 0 {
            return false;
        }
        if self.y < 0 {
            return false;
        }
        if self.x as f64 > (WIDTH as f64 - PIXEL_SIZE) {
            return false;
        }
        if self.y as f64 > (HEIGHT as f64 - PIXEL_SIZE) {
            return false;
        }

        true
    }
}
