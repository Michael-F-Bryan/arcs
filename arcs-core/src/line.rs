use crate::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Self { Line { start, end } }

    pub fn displacement(&self) -> Vector { self.end - self.start }

    pub fn direction(&self) -> Vector { self.displacement().unit_vector() }
}
