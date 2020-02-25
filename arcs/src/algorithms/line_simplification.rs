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
    // TODO: replace this with `if let [first, rest @ .., last]` in rust 1.42
    if points.len() < 2 {
        return;
    }
    let first = points.first().unwrap();
    let last = points.last().unwrap();
    let rest = &points[1..points.len() - 1];

    let line_segment = Line::new(*first, *last);

    if let Some((ix, distance)) =
        max_by_key(rest, |p| line_segment.perpendicular_distance_to(*p))
    {
        if distance > tolerance {
            // note: index is the index into `rest`, but we want it relative
            // to `point`
            let ix = ix + 1;

            simplify_points(&points[..=ix], tolerance, buffer);
            buffer.push(points[ix]);
            simplify_points(&points[ix..], tolerance, buffer);
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
    fn empty_line() {
        let points: Vec<Point> = Vec::new();

        let got = simplify(&points, Length::new(1.0));

        assert!(got.is_empty());
    }

    #[test]
    fn line_with_one_point() {
        let points = vec![Point::new(0.0, 0.0)];

        let got = simplify(&points, Length::new(1.0));

        assert_eq!(got, points);
    }

    #[test]
    fn line_with_two_points() {
        let points = vec![Point::new(0.0, 0.0), Point::new(10.0, 2.0)];

        let got = simplify(&points, Length::new(1.0));

        assert_eq!(got, points);
    }

    #[test]
    fn simplify_a_straight_line_to_two_points() {
        let points: Vec<Point> =
            (0..100).map(|i| Point::new(i as f64, 0.0)).collect();
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

    #[test]
    fn simplify_more_realistic_line() {
        // Found by drawing it out on paper and using a ruler to determine
        // point coordinates
        let line = vec![
            Point::new(-43.0, 8.0),
            Point::new(-24.0, 19.0),
            Point::new(-13.0, 23.0),
            Point::new(-8.0, 36.0),
            Point::new(7.0, 40.0),
            Point::new(24.0, 12.0),
            Point::new(44.0, -6.0),
            Point::new(57.0, 2.0),
            Point::new(70.0, 7.0),
        ];
        let should_be = vec![line[0], line[4], line[6], line[8]];
        let ruler_width = Length::new(20.0);

        let got = simplify(&line, ruler_width / 2.0);

        assert_eq!(got, should_be);
    }
}
