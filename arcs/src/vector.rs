use kurbo::{Affine, Vec2};

pub enum CanvasSpace {}

pub enum DrawingSpace {}

pub type Vector = euclid::Vector2D<f64, DrawingSpace>;
pub type Transform = euclid::Transform2D<f64, DrawingSpace, DrawingSpace>;

/// How something may be oriented.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    Anticlockwise,
    Collinear,
}

impl Orientation {
    pub fn of(first: Vector, second: Vector, third: Vector) -> Orientation {
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

    let x =
        (bc * (second.y - third.y) - cd * (first.y - second.y)) / determinant;
    let y =
        ((first.x - second.x) * cd - (second.x - third.x) * bc) / determinant;

    Some(Vector::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

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
