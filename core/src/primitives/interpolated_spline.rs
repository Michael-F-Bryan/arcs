#![allow(missing_docs)]

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedSpline {
    x_segments: Vec<Cubic>,
    y_segments: Vec<Cubic>,
}

impl InterpolatedSpline {
    pub fn point_at(&self, t: f64) -> (f64, f64) {
        debug_assert!(
            0.0 <= t && t <= 1.0,
            "{} should be a fraction between 0 and 1, inclusive",
            t,
        );

        let segment_number = t * self.len() as f64;
        let ix = segment_number.floor() as usize;
        let t = segment_number.fract();

        let x = self.x_segments[ix].evaluate(t);
        let y = self.y_segments[ix].evaluate(t);

        (x, y)
    }

    pub fn len(&self) -> usize { self.x_segments.len() }
}

trait Polynomial {
    type Derivative: Polynomial;

    fn evaluate(&self, t: f64) -> f64;
    fn derive(&self) -> Self::Derivative;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Constant {
    coefficients: [f64; 1],
}

impl Polynomial for Constant {
    type Derivative = Constant;

    fn evaluate(&self, _t: f64) -> f64 { self.coefficients[0] }

    fn derive(&self) -> Self::Derivative {
        Constant {
            coefficients: [0.0],
        }
    }
}

/// A helper that generates a newtype'd array that can be used as a
/// [`Polynomial`].
macro_rules! strongly_typed_polynomial {
    ($name:ident, $degree:expr => $derivative:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct $name {
            coefficients: [f64; $degree + 1],
        }

        #[allow(dead_code)]
        impl $name {
            const fn with_coefficients(
                coefficients: [f64; $degree + 1],
            ) -> Self {
                $name { coefficients }
            }

            const fn identity() -> Self {
                let mut coefficients = [0.0; $degree + 1];
                coefficients[$degree] = 1.0;
                $name::with_coefficients(coefficients)
            }
        }

        impl Polynomial for $name {
            type Derivative = $derivative;

            fn evaluate(&self, t: f64) -> f64 {
                evaluate_polynomial(t, &self.coefficients)
            }

            fn derive(&self) -> Self::Derivative {
                let mut derivative = $derivative {
                    coefficients: Default::default(),
                };
                derive(&self.coefficients, &mut derivative.coefficients);

                derivative
            }
        }
    };
}

strongly_typed_polynomial!(Linear, 1 => Constant);
strongly_typed_polynomial!(Quadratic, 2 => Linear);
strongly_typed_polynomial!(Cubic, 3 => Quadratic);

fn evaluate_polynomial(t: f64, coefficients: &[f64]) -> f64 {
    let mut sum = 0.0;

    for (i, coefficient) in coefficients.iter().enumerate() {
        sum += coefficient * t.powi(i as i32);
    }

    sum
}

fn derive(coefficients: &[f64], out: &mut [f64]) {
    debug_assert_eq!(out.len() + 1, coefficients.len());

    for i in 0..out.len() {
        out[i] = coefficients[i + 1] * (i as f64 + 1.0);
    }
}

fn approximate<P>(
    lower_bound: f64,
    upper_bound: f64,
    tolerance: f64,
    poly: &P,
) -> impl Iterator<Item = (f64, f64)> + '_
where
    P: Polynomial,
{
    std::iter::once((lower_bound, poly.evaluate(lower_bound))).chain(
        Approximate {
            current: lower_bound,
            tolerance,
            upper_bound,
            polynomial: poly,
            derivative: poly.derive(),
        },
    )
}

#[derive(Debug, Clone, PartialEq)]
struct Approximate<'a, P: Polynomial> {
    current: f64,
    tolerance: f64,
    upper_bound: f64,
    polynomial: &'a P,
    derivative: P::Derivative,
}

impl<'a, P: Polynomial> Approximate<'a, P> {
    fn eval(&self, t: f64) -> f64 { self.polynomial.evaluate(t) }

    fn error(&self, step_size: f64) -> f64 {
        let t_1 = self.current + step_size;
        let x_1 = self.eval(t_1);
        let linear_approximation =
            line_through((self.current, self.eval(self.current)), (t_1, x_1));

        let midpoint = (t_1 + self.current) / 2.0;
        let error =
            self.eval(midpoint) - linear_approximation.evaluate(midpoint);

        error.abs()
    }

    fn next_step(&self) -> f64 {
        const EPSILON: f64 = 0.0001;

        debug_assert!(self.current < self.upper_bound);

        let t_0 = self.current;
        let x_0 = self.eval(t_0);

        // first, keep increasing the step size until our error is greater than
        // the tolerance

        let mut step_size_upper_bound = self.tolerance;
        let max_step_size = self.upper_bound - self.current;

        while self.error(step_size_upper_bound) <= self.tolerance
            && step_size_upper_bound < max_step_size
        {
            step_size_upper_bound *= 2.0;
        }

        step_size_upper_bound = step_size_upper_bound.min(max_step_size);

        // then do a binary search to find a "good enough" step size
        let mut practically_at_upper_limit = false;
        let direction = |step_size: f64| {
            let e = self.error(step_size);

            if e <= self.tolerance
                && (step_size - step_size_upper_bound).abs() < EPSILON
            {
                // the edge case where you are *really* close to the upper
                // limit, but not quite on it
                practically_at_upper_limit = true;
                Ordering::Equal
            } else if e <= 0.9 * self.tolerance {
                Ordering::Greater
            } else if e >= self.tolerance {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        };

        let step_size =
            binary_search(EPSILON, step_size_upper_bound, direction);

        if practically_at_upper_limit {
            self.upper_bound
        } else {
            self.current + step_size
        }
    }
}

fn binary_search<F>(lower: f64, upper: f64, mut f: F) -> f64
where
    F: FnMut(f64) -> Ordering,
{
    let mut lower = lower;
    let mut upper = upper;
    let mut midpoint = (lower + upper) / 2.0;

    while lower < upper {
        midpoint = (lower + upper) / 2.0;
        let direction = f(midpoint);

        match direction {
            Ordering::Less => {
                upper = midpoint;
            },
            Ordering::Greater => {
                lower = midpoint;
            },
            Ordering::Equal => return midpoint,
        }
    }

    midpoint
}

impl<'a, P> Iterator for Approximate<'a, P>
where
    P: Polynomial,
{
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.upper_bound {
            return None;
        }

        let t = self.next_step();

        self.current = t;
        Some((t, self.polynomial.evaluate(t)))
    }
}

fn line_through(first: (f64, f64), second: (f64, f64)) -> Linear {
    let (t_0, x_0) = first;
    let (t_1, x_1) = second;

    debug_assert!(t_0 < t_1);

    let m = (x_1 - x_0) / (t_1 - t_0);
    let c = x_0 - m * t_0;

    Linear::with_coefficients([c, m])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_quadratic_to_linear() {
        // x(t) = 5 + 2 t - 3 t^2
        // x'(t) = 2 - 6 t
        let quadratic = Quadratic {
            coefficients: [5.0, 2.0, -3.0],
        };
        let should_be = [2.0, -6.0];

        let got = quadratic.derive();

        assert_eq!(got.coefficients, should_be);
    }

    #[test]
    fn derive_cubic_to_quadratic() {
        // x(t) = 5 + 2 t - 3 t^2 + t^3
        // x'(t) = 2 - 6 t + 3 t^2
        let cubic = Cubic::with_coefficients([5.0, 2.0, -3.0, 1.0]);
        let should_be = [2.0, -6.0, 3.0];

        let got = cubic.derive();

        assert_eq!(got.coefficients, should_be);
    }

    #[test]
    fn evaluate_a_simple_cubic() {
        let inputs = &[0.0, 1.0, 2.0, 4.0, -3.0, 1.5];
        let poly = Cubic::identity();

        for &t in inputs {
            let got = poly.evaluate(t);
            let should_be = t.powi(3);
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn each_approximated_point_is_on_the_curve() {
        let cubic = Cubic::with_coefficients([5.0, 2.0, -3.0, 1.0]);
        let lower = 0.0;
        let upper = 3.0;
        let tolerance = 0.25;

        let got: Vec<_> =
            approximate(lower, upper, tolerance / 2.0, &cubic).collect();

        assert_eq!(got[0].0, lower);
        assert_eq!(got.last().unwrap().0, upper);
        for (t, x) in got {
            let should_be = cubic.evaluate(t);
            assert_eq!(x, should_be);
        }
    }

    #[test]
    #[ignore]
    fn everywhere_along_linear_segments_is_within_tolerance() {
        let cubic = Cubic::with_coefficients([5.0, 2.0, -3.0, 1.0]);
        let lower = 0.0;
        let upper = 3.0;
        let tolerance = 0.25;

        let got: Vec<_> =
            approximate(lower, upper, tolerance / 2.0, &cubic).collect();

        assert_eq!(got[0].0, lower);
        assert_eq!(got.last().unwrap().0, upper);

        for window in got.windows(2) {
            let start = window[0];
            let end = window[1];
            let line = line_through(start, end);

            for step in 0..50 {
                let t = start.0 + (end.0 - start.0) * step as f64;
                let approximate_location = line.evaluate(t);
                let actual = cubic.evaluate(t);

                let error = (actual - approximate_location).abs();
                assert!(
                    error <= tolerance,
                    "{:?} has an error of {} at ({}, {})",
                    window,
                    error,
                    t,
                    actual,
                );
            }
        }
    }
}
