extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::*;
use piston::input::*;
use piston::{
    window::WindowSettings, Button, ButtonEvent, ButtonState, EventLoop, EventSettings, Events,
    RenderArgs, RenderEvent, UpdateEvent,
};

const PIXEL_SIZE: f64 = 20.0;
const FPS: u64 = 60;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Game {
    gl: GlGraphics,
    snake: Snake,
}
impl Game {
    fn render(&mut self, args: &RenderArgs) {
        let gray: [f32; 4] = [0.6, 0.6, 0.6, 1.0];

        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(gray, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        if self.snake.is_alive() {
            self.snake.update();
        }
    }

    fn handle_button(&mut self, btn: &Button) {
        let last_heading = self.snake.heading.clone();

        self.snake.heading = match btn {
            Button::Keyboard(Key::Up) if last_heading != Direction::Down => Direction::Up,
            Button::Keyboard(Key::Down) if last_heading != Direction::Up => Direction::Down,
            Button::Keyboard(Key::Left) if last_heading != Direction::Right => Direction::Left,
            Button::Keyboard(Key::Right) if last_heading != Direction::Left => Direction::Right,
            _ => last_heading,
        }
    }
}

struct Snake {
    x: u32,
    y: u32,
    heading: Direction,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.x as f64, self.y as f64, PIXEL_SIZE);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(green, square, transform, gl);
        });
    }

    fn update(&mut self) {
        // TODO: Wall...?
        match self.heading {
            Direction::Up => self.y -= 1, // FIXME: Broke here, add wall check
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1, // FIXME: Broke here, add wall check
            Direction::Right => self.x += 1,
        }
    }

    fn is_alive(&self) -> bool {
        if &self.x < &0 {
            return false;
        }
        if &self.y < &0 {
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

fn main() {
    let open_gl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake game...!", [WIDTH, HEIGHT])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(open_gl),
        snake: Snake {
            x: 0,
            y: 0,
            heading: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(FPS);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if e.update_args().is_some() {
            game.update();
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.handle_button(&args.button)
            }
        }
    }
}
