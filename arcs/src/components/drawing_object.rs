use crate::primitives::{Arc, Line, Point};
use specs::prelude::*;
use specs_derive::Component;

// for rustdoc links
#[allow(unused_imports)]
use crate::components::Layer;

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct DrawingObject {
    pub geometry: Geometry,
    /// The [`Layer`] this [`DrawingObject`] is attached to.
    pub layer: Entity,
}

/// The geometry of a [`DrawingObject`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Geometry {
    Line(Line),
    Arc(Arc),
    Point(Point),
}
