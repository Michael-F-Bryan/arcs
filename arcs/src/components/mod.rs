//! Common components used by the `arcs` CAD library.

mod dimension;
mod drawing_object;
mod layer;
mod name;
mod styles;

pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable};
pub use styles::{LineStyle, PointStyle};

use specs::{World, WorldExt};

/// Register all [`specs::Components`].
pub fn register(world: &mut World) {
    world.register::<DrawingObject>();
    world.register::<crate::algorithms::BoundingBox>();
    world.register::<Name>();
    world.register::<LineStyle>();
    world.register::<PointStyle>();
    world.register::<Layer>();
}
