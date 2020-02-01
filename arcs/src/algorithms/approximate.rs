use crate::{Angle, Arc, Line, Point};
use std::{
    iter,
    iter::{Chain, Once},
};

/// Approximate a shape with a bunch of [`Point`]s.
pub trait Approximate {
    /// An iterator over the approximated vertices.
    type Iter: Iterator<Item = Point>;

    /// Approximate the shape, ensuring the resulting path is within `tolerance`
    /// units of the original.
    fn approximate(&self, tolerance: f64) -> Self::Iter;
}

impl<'a, A: Approximate + ?Sized> Approximate for &'a A {
    type Iter = A::Iter;

    fn approximate(&self, tolerance: f64) -> Self::Iter {
        (*self).approximate(tolerance)
    }
}

impl Approximate for Point {
    type Iter = Once<Point>;

    fn approximate(&self, _tolerance: f64) -> Self::Iter {
        std::iter::once(*self)
    }
}

impl Approximate for Line {
    type Iter = Chain<Once<Point>, Once<Point>>;

    fn approximate(&self, _tolerance: f64) -> Self::Iter {
        iter::once(self.start).chain(iter::once(self.end))
    }
}

impl Approximate for Arc {
    type Iter = ApproximatedArc;

    fn approximate(&self, tolerance: f64) -> Self::Iter {
        // Draw a chord between points A and B on a circle with centre C.
        // Draw a line which bisects the angle ACB and intersects with the
        // chord at point D.
        // The distance from D to the arc is our "quality"
        // (i.e. |CD| + quality = radius).
        //
        // From the triangle DCB:
        //   cos(θ/2) = |CD|/R
        //   cos(θ/2) = 1 - quality/R
        //
        //  where θ is the angle swept by a chord with the desired "quality".
        //
        // # line segments to approximate with the specified quality:
        //   N = ⌈SweepAngle/θ⌉

        let (steps, delta) = if tolerance <= 0.0 || self.radius() <= tolerance {
            (1, self.sweep_angle())
        } else {
            let cos_theta_on_two = 1.0 - tolerance / self.radius();
            let theta = cos_theta_on_two.acos() * 2.0;
            let line_segment_count = self.sweep_angle().get() / theta;

            // make sure we always have at least 2 points
            let line_segment_count = f64::max(line_segment_count, 2.0);
            let actual_step = self.sweep_angle() / line_segment_count;

            (line_segment_count.ceil().abs() as usize, actual_step)
        };

        ApproximatedArc {
            i: 0,
            steps,
            step_size: delta,
            arc: *self,
        }
    }
}

/// An iterator over the points in an arc approximation.
///
/// This shouldn't be used directly, you are probably looking for
/// `Arc::approximate()`.
#[derive(Debug, Clone)]
#[allow(missing_copy_implementations)] // iterators which are Copy are a footgun
pub struct ApproximatedArc {
    i: usize,
    steps: usize,
    step_size: Angle,
    arc: Arc,
}

impl Iterator for ApproximatedArc {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.steps {
            return None;
        }

        let angle = Angle::radians(self.i as f64 * self.step_size.radians);
        let point = self.arc.point_at(angle);
        self.i += 1;
        Some(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximate_arc_with_points() {
        let arc = Arc::from_centre_radius(
            Point::zero(),
            100.0,
            Angle::zero(),
            Angle::frac_pi_2(),
        );
        let quality = 10.0;

        let pieces: Vec<_> = arc.approximate(quality).collect();

        for &piece in &pieces {
            let error = arc.radius() - (piece - arc.centre()).length();
            assert!(error < quality);
        }
        assert_eq!(arc.start(), *pieces.first().unwrap());
        assert_eq!(arc.end(), *pieces.last().unwrap());
    }
}
