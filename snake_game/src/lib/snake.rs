use crate::lib::shared::{Direction, HEIGHT, PIXEL_SIZE, WIDTH};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::lib::colors;
use crate::lib::shared;

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub heading: Direction,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        for (x, y) in &self.body[0..self.body.len() - 1] {
            let square = shared::square_from_coordinates(x, y);

            gl.draw(args.viewport(), |c, gl| {
                graphics::rectangle(colors::FADED_GREEN, square, c.transform, gl);
            });
        }

        let (x, y) = self.snake_head();
        let square = shared::square_from_coordinates(x, y);
        gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(colors::GREEN, square, c.transform, gl);
        });
    }

    pub fn update(&mut self) {
        // TODO: Wall...?

        let (head_x, head_y) = self.snake_head();
        let next_head = match self.heading {
            Direction::Up => (*head_x, head_y - 1),
            Direction::Down => (*head_x, head_y + 1),
            Direction::Left => (head_x - 1, *head_y),
            Direction::Right => (head_x + 1, *head_y),
            Direction::StandBy => (*head_x, *head_y),
        };

        if self.heading != Direction::StandBy {
            self.body.push(next_head);
            self.body.remove(0);
        }
    }

    fn snake_head(&self) -> &(i32, i32) {
        self.body.last().expect("PANIC! - Snake head is None")
    }

    pub fn is_alive(&self) -> bool {
        let (x, y) = self.snake_head();

        if x < &0 {
            return false;
        }
        if y < &0 {
            return false;
        }
        if (*x as f64) * PIXEL_SIZE > (WIDTH as f64) {
            return false;
        }
        if (*y as f64) * PIXEL_SIZE > (HEIGHT as f64) {
            return false;
        }

        true
    }
}
