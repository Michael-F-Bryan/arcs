use crate::primitives::{Arc, Line, Point};
use specs::prelude::*;

// for rustdoc links
#[allow(unused_imports)]
use crate::components::Layer;

/// Something which can be drawn on the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawingObject {
    pub geometry: Geometry,
    /// The [`Layer`] this [`DrawingObject`] is attached to.
    pub layer: Entity,
}

impl Component for DrawingObject {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

decl_component!(DrawingObject);

/// The geometry of a [`DrawingObject`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Geometry {
    Line(Line),
    Arc(Arc),
    Point(Point),
}
