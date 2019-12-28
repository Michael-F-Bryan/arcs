use crate::{
    primitives::{Line, Point},
    Vector,
};
use std::iter::FromIterator;

pub trait ClosestPoint {
    fn closest_point(&self, target: Vector) -> Closest;
}

impl ClosestPoint for Vector {
    fn closest_point(&self, _target: Vector) -> Closest { Closest::One(*self) }
}

impl ClosestPoint for Point {
    fn closest_point(&self, _target: Vector) -> Closest {
        Closest::One(self.location)
    }
}

impl ClosestPoint for Line {
    fn closest_point(&self, target: Vector) -> Closest {
        if self.length() == 0.0 {
            return Closest::One(self.start);
        }

        let start = self.start;
        let displacement = self.displacement();

        // equation of the line: start + t * displacement, where 0 <= t <= 1

        let t = Vector::dot(target - start, displacement)
            / (self.length() * self.length());

        Closest::One(if t <= 0.0 {
            self.start
        } else if t >= 1.0 {
            self.end
        } else {
            start + t * displacement
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Closest {
    Infinite,
    One(Vector),
    Many(Vec<Vector>),
}

impl Closest {
    pub fn is_infinite(&self) -> bool {
        match self {
            Closest::Infinite => true,
            _ => false,
        }
    }

    pub fn points(&self) -> &[Vector] {
        match self {
            Closest::Infinite => &[],
            Closest::One(item) => std::slice::from_ref(item),
            Closest::Many(items) => &items,
        }
    }
}

impl FromIterator<Vector> for Closest {
    fn from_iter<I: IntoIterator<Item = Vector>>(iter: I) -> Closest {
        let items = Vec::from_iter(iter);

        match items.len() {
            0 => Closest::Infinite,
            1 => Closest::One(items[0]),
            _ => Closest::Many(items),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_the_line() {
        let start = Vector::new(1.0, 2.0);
        let end = Vector::new(3.0, 10.0);
        let line = Line::new(start, end);
        let midpoint = (start + end) / 2.0;

        let got = line.closest_point(midpoint);

        assert_eq!(got, Closest::One(midpoint));
    }

    #[test]
    fn closest_point_to_zero_length_line() {
        let start = Vector::new(1.0, 2.0);
        let line = Line::new(start, start);
        assert_eq!(line.length(), 0.0);
        let target = Vector::new(10.0, 0.0);

        let got = line.closest_point(target);

        assert_eq!(got, Closest::One(start));
    }

    #[test]
    fn away_from_the_line() {
        let start = Vector::new(0.0, 0.0);
        let end = Vector::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Vector::new(5.0, 5.0));

        assert_eq!(got, Closest::One(Vector::new(5.0, 0.0)));
    }

    #[test]
    fn past_the_end_of_the_line() {
        let start = Vector::new(0.0, 0.0);
        let end = Vector::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Vector::new(15.0, 5.0));

        assert_eq!(got, Closest::One(end));
    }

    #[test]
    fn before_the_start_of_the_line() {
        let start = Vector::new(0.0, 0.0);
        let end = Vector::new(10.0, 0.0);
        let line = Line::new(start, end);

        let got = line.closest_point(Vector::new(-5.0, 5.0));

        assert_eq!(got, Closest::One(start));
    }
}
