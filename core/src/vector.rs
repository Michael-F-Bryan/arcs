use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

/// Your typical 2D vector.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    /// Create a new [`Vector`].
    ///
    /// # Panics
    ///
    /// This will panic
    pub fn new(x: f64, y: f64) -> Self {
        assert!(x.is_finite(), "Can't create a vector with {}", x);
        assert!(y.is_finite(), "Can't create a vector with {}", y);

        Vector::new_unchecked(x, y)
    }

    pub const fn new_unchecked(x: f64, y: f64) -> Self { Vector { x, y } }

    pub fn from_r_theta(radius: f64, angle: f64) -> Self {
        Vector::new(radius * angle.cos(), radius * angle.sin())
    }

    pub const fn zero() -> Vector { Vector::new_unchecked(0.0, 0.0) }

    pub const fn x_axis() -> Vector { Vector::new_unchecked(1.0, 0.0) }

    pub const fn y_axis() -> Vector { Vector::new_unchecked(0.0, 1.0) }

    pub fn length(self) -> f64 { self.x.hypot(self.y) }

    pub fn angle(self) -> f64 { f64::atan2(self.y, self.x) }

    pub fn unit_vector(self) -> Vector {
        let magnitude = self.length();
        if magnitude == 0.0 {
            Vector::zero()
        } else {
            self / magnitude
        }
    }

    pub fn orientation(
        first: Vector,
        second: Vector,
        third: Vector,
    ) -> Orientation {
        let value = (second.y - first.y) * (third.x - second.x)
            - (second.x - first.x) * (third.y - second.y);

        if value > 0.0 {
            Orientation::Clockwise
        } else if value < 0.0 {
            Orientation::Anticlockwise
        } else {
            Orientation::Collinear
        }
    }

    pub fn dot(left: Vector, right: Vector) -> f64 {
        left.x * right.x + left.y * right.y
    }

    pub fn cross(left: Vector, right: Vector) -> f64 {
        left.x * right.y - right.x * left.y
    }

    pub fn lerp(start: Vector, end: Vector, progress: f64) -> Vector {
        start + (end - start) * progress
    }

    pub fn centre_of_three_points(
        first: Vector,
        second: Vector,
        third: Vector,
    ) -> Option<Vector> {
        let temp = Vector::dot(second, second);
        let bc = (Vector::dot(first, first) - temp) / 2.0;
        let cd = (temp - third.x * third.x - third.y * third.y) / 2.0;
        let determinant = (first.x - second.x) * (second.y - third.y)
            - (second.x - third.x) * (first.y - second.y);

        if determinant == 0.0 {
            // the points are collinear
            return None;
        }

        let x = (bc * (second.y - third.y) - cd * (first.y - second.y))
            / determinant;
        let y = ((first.x - second.x) * cd - (second.x - third.x) * bc)
            / determinant;

        Some(Vector::new(x, y))
    }

    pub fn rotated(self, angle: f64) -> Vector {
        Vector::from_r_theta(self.length(), self.angle() + angle)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) { *self = *self + other; }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) { *self = *self - other; }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector { other * self }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) { *self = *self * other; }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        assert!(other.is_normal(), "Unable to divide by {}", other);
        Vector::new_unchecked(self.x / other, self.y / other)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) { *self = *self / other; }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    Anticlockwise,
    Collinear,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_vectors() {
        let left = Vector::new(1.0, 2.0);
        let right = Vector::new(-20.0, 2.5);
        let expected = Vector::new(-19.0, 4.5);

        let got = left + right;

        assert_eq!(got, expected);
    }

    #[test]
    fn subtract_two_vectors() {
        let left = Vector::new(1.0, 2.0);
        let right = Vector::new(-20.0, 2.5);
        let expected = Vector::new(1.0 - -20.0, 2.0 - 2.5);

        let got = left - right;

        assert_eq!(got, expected);
    }

    #[test]
    fn multiply_by_two() {
        let left = Vector::new(-20.0, 2.5);
        let expected = Vector::new(-20.0 * 2.0, 2.5 * 2.0);

        let got = left * 2.0;

        assert_eq!(got, expected);
    }

    #[test]
    fn divide_by_two() {
        let left = Vector::new(-20.0, 2.5);
        let expected = Vector::new(-20.0 / 2.0, 2.5 / 2.0);

        let got = left / 2.0;

        assert_eq!(got, expected);
    }

    #[test]
    #[should_panic(expected = "divide by 0")]
    fn divide_by_zero() {
        let left = Vector::new(-20.0, 2.5);

        let _ = left / 0.0;
    }

    #[test]
    fn find_centre_of_three_points() {
        let a = Vector::new(1.0, 0.0);
        let b = Vector::new(-1.0, 0.0);
        let c = Vector::new(0.0, 1.0);

        let centre = Vector::centre_of_three_points(a, b, c).unwrap();

        assert_eq!(centre, Vector::zero());
    }

    #[test]
    fn find_a_quarter_of_the_way_between_points() {
        let start = Vector::new(0.0, 0.0);
        let end = Vector::new(40.0, 8.0);
        let expected = Vector::new(10.0, 2.0);

        let got = Vector::lerp(start, end, 0.25);

        assert_eq!(got, expected);
    }
}
