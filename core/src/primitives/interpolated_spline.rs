#![allow(missing_docs)]

pub trait Polynomial {
    type Derivative: Polynomial;

    fn evaluate(&self, t: f64) -> f64;
    fn derive(&self) -> Self::Derivative;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Constant {
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
        pub struct $name {
            coefficients: [f64; $degree + 1],
        }

        impl $name {
            pub const fn with_coefficients(
                coefficients: [f64; $degree + 1],
            ) -> Self {
                $name { coefficients }
            }

            pub const fn identity() -> Self {
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
        let cubic = Cubic {
            coefficients: [5.0, 2.0, -3.0, 1.0],
        };
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
}
