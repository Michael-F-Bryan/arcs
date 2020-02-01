pub enum CanvasSpace {}

pub enum DrawingSpace {}

pub type Vector = euclid::Vector2D<f64, DrawingSpace>;
pub type Transform = euclid::Transform2D<f64, DrawingSpace, DrawingSpace>;
pub type Angle = euclid::Angle<f64>;
pub type Point = euclid::Point2D<f64, DrawingSpace>;
pub type Length = euclid::Length<f64, DrawingSpace>;

/// How something may be oriented.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,
    Anticlockwise,
    Collinear,
}

impl Orientation {
    pub fn of(first: Point, second: Point, third: Point) -> Orientation {
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
    first: Point,
    second: Point,
    third: Point,
) -> Option<Point> {
    // it's easier to do the math using vectors, but for semantic correctness we
    // accept points
    let first = first.to_vector();
    let second = second.to_vector();
    let third = third.to_vector();

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

    Some(Point::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_centre_of_three_points() {
        let a = Point::new(1.0, 0.0);
        let b = Point::new(-1.0, 0.0);
        let c = Point::new(0.0, 1.0);

        let centre = centre_of_three_points(a, b, c).unwrap();

        assert_eq!(centre, Vector::zero());
    }
}
