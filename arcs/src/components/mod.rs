//! Common components used by the `arcs` CAD library.

mod dimension;
mod drawing_object;
mod layer;
mod name;
mod styles;

pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable, NameTableBookkeeping};
pub use styles::{LineStyle, PointStyle};

use specs::{DispatcherBuilder, World, WorldExt};

/// Register any necessary background tasks with a [`DispatcherBuilder`].
pub fn register_background_tasks<'a, 'b>(
    builder: DispatcherBuilder<'a, 'b>,
    world: &World,
) -> DispatcherBuilder<'a, 'b> {
    builder.with(
        NameTableBookkeeping::new(world),
        NameTableBookkeeping::NAME,
        &[],
    )
}

/// Register all [`specs::Components`].
pub fn register(world: &mut World) {
    world.register::<DrawingObject>();
    world.register::<crate::algorithms::BoundingBox>();
    world.register::<Name>();
    world.register::<LineStyle>();
    world.register::<PointStyle>();
    world.register::<Layer>();
}
