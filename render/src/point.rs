use crate::px;

/// A 2D location on the screen, in pixels.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: px,
    pub y: px,
}

impl Point {
    pub const fn new(x: px, y: px) -> Self { Point { x, y } }

    pub const fn zero() -> Point { Point::new(px::new(0), px::new(0)) }
}
