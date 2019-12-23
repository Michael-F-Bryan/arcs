use crate::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
}

impl Line {
    pub const fn new(start: Vector, end: Vector) -> Self { Line { start, end } }

    pub fn displacement(&self) -> Vector { self.end - self.start }

    pub fn direction(&self) -> Vector { self.displacement().unit_vector() }

    pub fn length(self) -> f64 { self.displacement().length() }

    pub fn perpendicular_distance_to(self, point: Vector) -> f64 {
        let side_a = self.start - point;
        let side_b = self.end - point;
        let area = Vector::cross(side_a, side_b) / 2.0;

        // area = base * height / 2
        let base_length = self.length();

        if base_length.abs() < 0.0001 {
            side_a.length()
        } else {
            area * 2.0 / base_length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_length() {
        let start = Vector::new(1.0, 1.0);
        let displacement = Vector::new(3.0, 4.0);
        let v = Line::new(start, start + displacement);

        assert_eq!(v.length(), 5.0);
        assert_eq!(v.displacement(), displacement);
    }
}
