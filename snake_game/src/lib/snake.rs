use crate::lib::shared::Direction;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::lib::colors;
use crate::lib::shared;

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub heading: Direction,
}

fn correct_next_head(next_head: &mut (i32, i32)) {
    let (x, y) = next_head;

    if *x < 0 {
        *x = shared::MAX_X;
    }
    if *x > shared::MAX_X {
        *x = 0;
    }

    if *y < 0 {
        *x = shared::MAX_Y;
    }
    if *y > shared::MAX_Y {
        *y = 0;
    }
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
        let (head_x, head_y) = self.snake_head();
        let mut next_head = match self.heading {
            Direction::Up => (*head_x, head_y - 1),
            Direction::Down => (*head_x, head_y + 1),
            Direction::Left => (head_x - 1, *head_y),
            Direction::Right => (head_x + 1, *head_y),
            Direction::StandBy => (*head_x, *head_y),
        };

        correct_next_head(&mut next_head);

        if self.heading != Direction::StandBy {
            self.body.push(next_head);
            self.body.remove(0);
        }
    }

    fn snake_head(&self) -> &(i32, i32) {
        self.body.last().expect("PANIC! - Snake head is None")
    }

    pub fn is_alive(&self) -> bool {
        // TODO: Die if the head touches its body

        true
    }
}
