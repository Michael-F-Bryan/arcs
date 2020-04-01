use crate::{Angle, Arc, Length, Line, Point, Vector};
use euclid::approxeq::ApproxEq;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Given three points, try to find the arc which would round the corner.
pub fn fillet_three_points(
    start: Point,
    corner: Point,
    end: Point,
    radius: Length,
) -> Result<Arc, FilletError> {
    let incoming = Line::new(start, corner);
    let outgoing = Line::new(corner, end);

    let angle_1 = incoming.displacement().angle_from_x_axis();
    let angle_2 = outgoing.displacement().angle_from_x_axis();

    let rotation_angle = (angle_2 - angle_1).signed();

    if rotation_angle.approx_eq(&Angle::zero())
        || rotation_angle.positive().approx_eq(&Angle::pi())
    {
        return Err(FilletError::CollinearLines);
    }

    // the amount of material to remove from each line can be determined by
    // bisecting the angle between incoming and outgoing, then doing pythagoras.
    // Filleting two lines using a perfect arc will always be symmetrical
    let half_angle: Angle = rotation_angle / 2.0;
    let mut length_to_remove: Length = radius * half_angle.get().tan().abs();

    let smallest_length = Length::new(
        incoming
            .displacement()
            .length()
            .min(outgoing.displacement().length()),
    );

    if length_to_remove.approx_eq(&smallest_length) {
        // as a special case if you're *just* too short, we'll shrink the fillet
        // a tad to allow for floating point errors
        length_to_remove = smallest_length;
    } else if length_to_remove > smallest_length {
        return Err(FilletError::InsufficientLength {
            required: length_to_remove,
            available: smallest_length,
        });
    }

    let start_point = corner - dbg!(incoming.direction() * length_to_remove.0);

    let start_to_centre = if rotation_angle >= Angle::zero() {
        r_theta(radius, angle_1 - Angle::frac_pi_2())
    } else {
        r_theta(radius, angle_1 + Angle::frac_pi_2())
    };

    let centre = dbg!(start_point) - dbg!(start_to_centre);

    Ok(Arc::from_centre_radius(
        centre,
        start_to_centre.length(),
        start_to_centre.angle_from_x_axis().positive(),
        rotation_angle,
    ))
}

fn r_theta(radius: Length, angle: Angle) -> Vector {
    let (sin, cos) = angle.sin_cos();
    Vector::from_lengths(radius * cos, radius * sin)
}

/// Errors that may be encountered when filleting (e.g.
/// [`fillet_three_points()`]).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FilletError {
    /// The two lines are collinear.
    CollinearLines,
    /// The edges aren't long enough to apply the desired fillet.
    InsufficientLength {
        /// The required length.
        required: Length,
        /// The length that is actually available.
        available: Length,
    },
}

impl Error for FilletError {}

impl Display for FilletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            FilletError::CollinearLines => {
                write!(f, "Unable to fillet collinear lines")
            },
            FilletError::InsufficientLength { required, available } => write!(f, "The edges are not long enough to fillet, expected {} but only {} was available", required, available)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fillet_90_degree_acw_corner() {
        let start = Point::new(0.0, 0.0);
        let corner = Point::new(100.0, 0.0);
        let end = Point::new(100.0, 100.0);
        let radius = 20.0;
        let should_be = Arc::from_centre_radius(
            Point::new(80.0, 20.0),
            radius,
            Angle::frac_pi_2() * 3.0,
            Angle::frac_pi_2(),
        );

        let got = fillet_three_points(start, corner, end, Length::new(radius))
            .unwrap();

        assert!(got.approx_eq(&should_be), "{:#?} != {:?}", got, should_be);
    }

    #[test]
    fn fillet_90_degree_cw_corner() {
        let start = Point::new(100.0, 100.0);
        let corner = Point::new(100.0, 0.0);
        let end = Point::new(0.0, 0.0);
        let radius = 20.0;
        let should_be = Arc::from_centre_radius(
            Point::new(80.0, 20.0),
            radius,
            Angle::zero(),
            -Angle::frac_pi_2(),
        );

        let got = fillet_three_points(start, corner, end, Length::new(radius))
            .unwrap();

        assert!(got.approx_eq(&should_be), "{:#?} != {:?}", got, should_be);
    }

    #[test]
    fn collinear_lines() {
        let start = Point::new(90.0, 0.0);
        let corner = Point::new(100.0, 0.0);
        let radius = Length::new(20.0);

        let err =
            fillet_three_points(start, corner, start, radius).unwrap_err();

        assert_eq!(err, FilletError::CollinearLines);
    }

    #[test]
    fn insufficient_length() {
        let start = Point::new(90.0, 0.0);
        let corner = Point::new(100.0, 0.0);
        let end = Point::new(100.0, 1000.0);
        let radius = Length::new(20.0);
        assert!((start - corner).length() < radius.get());

        let err = fillet_three_points(start, corner, end, radius).unwrap_err();

        match err {
            FilletError::InsufficientLength {
                required,
                available,
            } => {
                assert!(required.approx_eq(&radius));
                assert!(available.approx_eq(&Length::new(10.0)));
            },
            other => panic!("Unexpected error, {:?}", other),
        }
    }

    #[test]
    fn clockwise_top_right_corner() {
        let start = Point::new(0.0, 10.0);
        let corner = Point::new(10.0, 10.0);
        let end = Point::new(10.0, 0.0);
        let radius = 5.0;
        let should_be = Arc::from_centre_radius(
            Point::new(5.0, 5.0),
            radius,
            Angle::frac_pi_2(),
            -Angle::frac_pi_2(),
        );

        let got = fillet_three_points(start, corner, end, Length::new(radius))
            .unwrap();

        assert!(got.approx_eq(&should_be), "{:#?} != {:?}", got, should_be);
    }

    #[test]
    fn anticlockwise_top_left_corner() {
        let start = Point::new(10.0, 10.0);
        let corner = Point::new(0.0, 10.0);
        let end = Point::new(0.0, 0.0);
        let radius = 5.0;
        let should_be = Arc::from_centre_radius(
            Point::new(5.0, 5.0),
            radius,
            Angle::frac_pi_2(),
            Angle::frac_pi_2(),
        );

        let got = fillet_three_points(start, corner, end, Length::new(radius))
            .unwrap();

        assert!(got.approx_eq(&should_be), "{:#?} != {:?}", got, should_be);
    }
}
