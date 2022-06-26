pub const PIXEL_SIZE: f64 = 4.0;
pub const FPS: u64 = 60;
pub const MAX_X: i32 = 100;
pub const MAX_Y: i32 = 100;

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    StandBy,
}

pub fn square_from_coordinates(x: &i32, y: &i32) -> [f64; 4] {
    graphics::rectangle::square(
        (*x as f64) * PIXEL_SIZE,
        (*y as f64) * PIXEL_SIZE,
        PIXEL_SIZE,
    )
}

pub fn are_coordinates_collide((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> bool {
    x1 == x2 && y1 == y2
}
