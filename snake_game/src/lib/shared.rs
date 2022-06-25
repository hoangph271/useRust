pub const PIXEL_SIZE: f64 = 4.0;
pub const FPS: u64 = 60;
pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    StandBy,
}

pub fn square_from_coordinates (x: &i32, y: &i32) -> [f64; 4] {
    graphics::rectangle::square(
        (*x as f64) * PIXEL_SIZE,
        (*y as f64) * PIXEL_SIZE,
        PIXEL_SIZE,
    )
}
