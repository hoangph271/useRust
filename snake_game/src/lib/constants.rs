pub const PIXEL_SIZE: f64 = 20.0;
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
