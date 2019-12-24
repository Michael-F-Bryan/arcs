use crate::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub location: Vector,
}

impl Point {
    pub fn new(location: Vector) -> Self { Point { location } }
}
