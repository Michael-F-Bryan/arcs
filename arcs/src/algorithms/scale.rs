use crate::{
    primitives::{Arc, Line, Point},
    Vector,
};
use kurbo::Affine;

/// Something which can be scaled in *Drawing Space*
pub trait Scale {
    fn scale(&mut self, factor: f64);

    fn scaled(&self, factor: f64) -> Self
    where 
        Self: Sized + Clone,
        {
            let mut clone = self.clone();
            clone.scale(factor);

            clone
        }

}

impl<'t, T: Scale + ?Sized> Scale for &'t mut T {
    fn scale(&mut self, factor: f64) {
        (*self).scale(factor);
    }
}

impl Scale for Vector {
    fn scale(&mut self, factor: f64) {
        let scale = Affine::scale(factor);
        let new_pos = scale * *self;
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

impl Scale for Point {
    fn scale(&mut self, factor: f64) {
        self.location.scale(factor);
    }
}

impl Scale for Line {
    fn scale(&mut self, factor: f64) {
        self.start.scale(factor);
        self.end.scale(factor);
    }
}

impl Scale for Arc {
    fn scale(&mut self, factor: f64) {
        let mut centre = self.centre();
        centre.scale(factor);
        *self = Arc::from_centre_radius(
            centre, 
            self.radius() * factor, 
            self.start_angle(), 
            self.sweep_angle(),
        );   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let original = Vector::new(1.0, 1.0);
        let factor = 2.0;

        let actual = original.scaled(factor);
        let expected = Vector::new(2.0, 2.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn line() {
        let start = Vector::new(2.0, 4.0);
        let end = Vector::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor = 1.5;

        let actual = original.scaled(factor);
        let expected = Line::new(Vector::new(3.0, 6.0), Vector::new(4.5, -7.5));

        assert_eq!(actual, expected);
    }

    #[test]
    fn arc() {
        let centre = Vector::new(-1.4, 2.0);
        let radius = 5.0;
        let start_angle = 0.5;
        let sweep_angle = 1.0;
        let original = Arc::from_centre_radius(centre, radius, start_angle, sweep_angle);
        let factor = 2.0;

        let actual = original.scaled(factor);
        let expected = Arc::from_centre_radius(Vector::new(-2.8, 4.0), radius * factor, start_angle, sweep_angle);

        assert_eq!(actual, expected);
    }
}