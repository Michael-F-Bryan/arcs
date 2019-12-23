use crate::primitives::{Line, Arc, Point};
use specs::prelude::*;
use specs_derive::Component;

/// A visual object which should be drawn by the rendering system.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
#[non_exhaustive]
pub enum Visual {
    Line(Line),
    Arc(Arc),
    Point(Point),
}
