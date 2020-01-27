use crate::components::Dimension;
use piet::Color;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct PointStyle {
    pub colour: Color,
    pub radius: Dimension,
}

impl Default for PointStyle {
    fn default() -> PointStyle {
        PointStyle {
            colour: Color::BLACK,
            radius: Dimension::default(),
        }
    }
}

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct LineStyle {
    pub stroke: Color,
    pub width: Dimension,
}

impl Default for LineStyle {
    fn default() -> LineStyle {
        LineStyle {
            stroke: Color::BLACK,
            width: Dimension::default(),
        }
    }
}

#[derive(Debug, Clone, Component)]
#[storage(HashMapStorage)]
pub struct WindowStyle {
    pub background_colour: Color,
}

impl Default for WindowStyle {
    fn default() -> WindowStyle {
        WindowStyle {
            background_colour: Color::WHITE,
        }
    }
}
