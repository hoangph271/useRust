extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod lib;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::*;
use piston::{
    window::WindowSettings, ButtonEvent, ButtonState, EventLoop, EventSettings, Events,
    RenderEvent, UpdateEvent,
};

use lib::constants::{Direction, FPS, HEIGHT, WIDTH};
use lib::game::Game;
use lib::snake::Snake;

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
            heading: Direction::StandBy,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(FPS);
    while let Some(e) = events.next(&mut window) {
        if !game.snake.is_alive() {
            break;
        }

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
