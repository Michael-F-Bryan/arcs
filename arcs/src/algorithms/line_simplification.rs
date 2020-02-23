use crate::primitives::Line;
use euclid::{Length, Point2D};

/// Decimate a curve composed of line segments to a *"simpler"* curve with fewer
/// points.
///
/// The algorithm defines *"simpler"* based on the maximum distance
/// (`tolerance`) between the original curve and the simplified curve.
///
/// You may want to research the [Ramer–Douglas–Peucker algorithm][wiki] for
/// the exact details and assumptions that can be made.
///
/// [wiki]: https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
pub fn simplify<Space>(
    points: &[Point2D<f64, Space>],
    tolerance: Length<f64, Space>,
) -> Vec<Point2D<f64, Space>> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut buffer = Vec::new();

    // push the first point
    buffer.push(points[0]);
    // then simplify every point in between the start and end
    simplify_points(&points[..], tolerance, &mut buffer);
    // and finally the last one
    buffer.push(*points.last().unwrap());

    buffer
}

fn simplify_points<Space>(
    points: &[Point2D<f64, Space>],
    tolerance: Length<f64, Space>,
    buffer: &mut Vec<Point2D<f64, Space>>,
) {
    if let [first, rest @ .., last] = points {
        let line_segment = Line::new(*first, *last);

        if let Some((ix, distance)) =
            max_by_key(rest, |p| line_segment.perpendicular_distance_to(*p))
        {
            if distance > tolerance {
                simplify_points(&points[..=ix], tolerance, buffer);
                buffer.push(points[ix]);
                simplify_points(&points[ix..], tolerance, buffer);
            }
        }
    }
}

fn max_by_key<T, F, K>(items: &[T], mut key_func: F) -> Option<(usize, K)>
where
    F: FnMut(&T) -> K,
    K: PartialOrd,
{
    let mut best_so_far = None;

    for (i, item) in items.iter().enumerate() {
        let key = key_func(item);

        let is_better = match best_so_far {
            Some((_, ref best_key)) => key > *best_key,
            None => true,
        };

        if is_better {
            best_so_far = Some((i, key));
        }
    }

    best_so_far
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;
    use std::f64::consts::PI;

    #[test]
    fn simplify_a_straight_line_to_two_points() {
        let points: Vec<Point> =
            (0..100).map(|i| Point::new(i as f64, i as f64)).collect();
        let should_be = &[points[0], points[99]];

        let got = simplify(&points, Length::new(0.1));

        assert_eq!(got, should_be);
    }

    #[test]
    fn simplify_a_horizontal_line_with_small_amounts_of_vertical_jitter() {
        let max_jitter = 0.1;

        let points: Vec<Point> = (0..100)
            .map(|i| {
                let jitter = max_jitter * (i as f64 / 100.0 * PI).sin();
                Point::new(i as f64, jitter)
            })
            .collect();

        let should_be = &[points[0], points[99]];

        let got = simplify(&points, Length::new(max_jitter * 2.0));

        assert_eq!(got, should_be);
    }
}
