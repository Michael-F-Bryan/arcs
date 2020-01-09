//! Common components used by the `arcs` CAD library.

mod bounding_box;
mod dimension;
mod drawing_object;
mod layer;
mod name;
mod styles;
mod viewport;

pub use bounding_box::BoundingBox;
pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable};
pub use styles::{LineStyle, PointStyle, WindowStyle};
pub use viewport::Viewport;

use specs::{World, WorldExt};

/// Register all [`specs::Component`]s.
pub fn register(world: &mut World) {
    world.register::<DrawingObject>();
    world.register::<BoundingBox>();
    world.register::<Name>();
    world.register::<Layer>();
    world.register::<LineStyle>();
    world.register::<PointStyle>();
    world.register::<WindowStyle>();
    world.register::<Viewport>();
}
