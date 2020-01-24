use crate::{
    algorithms::{AffineTransformable},
    components::{BoundingBox},
};
use kurbo::Affine;

/// Something which can be scaled **non-uniform** in x and y directions in *Drawing Space*
pub trait ScaleNonUniform {
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64);

    fn scaled_non_uniform(&self, factor_x: f64, factor_y: f64) -> Self
    where 
        Self: Sized + Clone,
        {
            let mut clone = self.clone();
            clone.scale_non_uniform(factor_x, factor_y);

            clone
        }

}

impl<A: AffineTransformable> ScaleNonUniform for A {
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64){
        // TODO: Change to `Affine::scale_non_uniform()` after crates.io update
        self.transform(Affine::new([factor_x, 0.0, 0.0, factor_y, 0.0, 0.0]));
    }
}

impl ScaleNonUniform for BoundingBox {
    fn scale_non_uniform(&mut self, factor_x: f64, factor_y: f64) {
        *self = BoundingBox::new(
            self.bottom_left().scaled_non_uniform(factor_x, factor_y),
            self.top_right().scaled_non_uniform(factor_x, factor_y)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Line};
    use crate::algorithms::Translate;
    use crate::Vector;

    #[test]
    fn scale_vector() {
        let x = -1.0;
        let y = 5.0;
        let original = Vector::new(x, y);
        let factor_x = 2.0;
        let factor_y = 2.5;

        let actual = original.scaled_non_uniform(factor_x, factor_y);
        // known value
        let expected = Vector::new(x * factor_x, y * factor_y);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_vector_around_base() {
        let original = Vector::new(-1.0, 5.0);
        let factor_x = 2.0;
        let factor_y = 2.5;
        let base = Vector::new(2.0, 0.0);

        let expected = Vector::new(-4.0, 12.5);
        
        // We can either use explicit transformation methods:
        let mut transformed = original.translated(Vector::zero() - base);
        transformed.scale_non_uniform(factor_x, factor_y);
        transformed.translate(base);

        assert_eq!(transformed, expected);

        // Or compose an `Affine` and pass it directly to the `transform` method:
        // keep in mind that transforms get composed *in reverse execution order*
        let translate_to_origin = Affine::translate(Vector::zero() - base);
        let scale_non_uniform = Affine::new([factor_x, 0.0, 0.0, factor_y, 0.0, 0.0]);
        let translate_back = Affine::translate(base);
        let combined_transform = translate_back * scale_non_uniform * translate_to_origin;
        
        let transformed = original.transformed(combined_transform);

        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale_line() {
        let start = Vector::new(2.0, 4.0);
        let end = Vector::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor_x = 1.5;
        let factor_y = -2.0;

        let actual = original.scaled_non_uniform(factor_x, factor_y);
        let expected = Line::new(Vector::new(2.0 * factor_x, 4.0 * factor_y), Vector::new(3.0 * factor_x, -5.0 * factor_y));

        assert_eq!(actual, expected);
    }

    #[test]
    fn scale_line_around_base() {
        let start = Vector::new(2.0, 4.0);
        let end = Vector::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor_x = 1.5;
        let factor_y = -2.0;

        let expected = Line::new(Vector::new(1.75, -9.5), Vector::new(3.25, 8.5));
        
        // scale by line mid-point as reference
        let mid_point = start + original.displacement() * 0.5;
        let mut transformed = original.translated(Vector::zero() - mid_point);
        transformed.scale_non_uniform(factor_x, factor_y);
        transformed.translate(mid_point);

        assert_eq!(transformed, expected);
    }
}