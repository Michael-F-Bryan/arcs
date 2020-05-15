use euclid::{Point2D, Vector2D};

/// How something may be oriented.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    /// The points are arranged in a clockwise direction.
    Clockwise,
    /// The points are arranged in an anti-clockwise direction.
    Anticlockwise,
    /// The points are collinear.
    Collinear,
}

impl Orientation {
    /// Find the orientation of 3 [`Point`]s.
    pub fn of<S>(
        first: Point2D<f64, S>,
        second: Point2D<f64, S>,
        third: Point2D<f64, S>,
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
}

/// Find the centre of an arc which passes through 3 [`Point`]s.
///
/// # Note
///
/// If the points are collinear then the problem is ambiguous, the radius
/// effectively becomes infinite and our centre could be literally anywhere.
///
/// ```rust
/// # use arcs_core::Point;
/// let first = Point::new(0.0, 0.0);
/// let second = Point::new(1.0, 0.0);
/// let third = Point::new(25.0, 0.0);
///
/// let got = arcs_core::centre_of_three_points(first, second, third);
///
/// assert!(got.is_none());
/// ```
pub fn centre_of_three_points<S>(
    first: Point2D<f64, S>,
    second: Point2D<f64, S>,
    third: Point2D<f64, S>,
) -> Option<Point2D<f64, S>> {
    // it's easier to do the math using vectors, but for semantic correctness we
    // accept points
    let first = first.to_vector();
    let second = second.to_vector();
    let third = third.to_vector();

    let temp = Vector2D::dot(second, second);
    let bc = (Vector2D::dot(first, first) - temp) / 2.0;
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

    Some(Point2D::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use euclid::default::Point2D;

    #[test]
    fn find_centre_of_three_points() {
        let a = Point2D::new(1.0, 0.0);
        let b = Point2D::new(-1.0, 0.0);
        let c = Point2D::new(0.0, 1.0);

        let centre = centre_of_three_points(a, b, c).unwrap();

        assert_eq!(centre, Point2D::zero());
    }
}
