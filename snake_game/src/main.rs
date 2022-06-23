extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::*;
// use piston::input::*;
use piston::{window::WindowSettings, EventSettings, Events, RenderArgs, RenderEvent};

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
}

struct Snake {
    x: i32,
    y: i32,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.x as f64, self.y as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(red, square, transform, gl);
        });
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake game...!", [400, 400])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(open_gl),
        snake: Snake { x: 50, y: 50 },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
