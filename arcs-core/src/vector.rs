use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        debug_assert!(x.is_finite(), "Can't create a vector with {}", x);
        debug_assert!(y.is_finite(), "Can't create a vector with {}", y);

        Vector { x, y }
    }

    pub fn magnitude(self) -> f64 { self.x.hypot(self.y) }

    pub fn angle(self) -> f64 { self.y.atan2(self.x) }
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
        assert!(other.is_normal(), "Tried to divide by {}", other);
        Vector::new(self.x / other, self.y / other)
    }
}

impl Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector { other / self }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) { *self = *self / other; }
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
}
