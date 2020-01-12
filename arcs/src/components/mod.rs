//! Common components used by the `arcs` CAD library.

/// A macro which, coupled with [`crate::components::ComponentVtable`], allows
/// the CAD engine to do things like component discovery (for
/// [`crate::components::register()`]) and undo/redo.
#[macro_export]
macro_rules! decl_component {
    ($type:ty) => {
        inventory::submit! {
            $crate::components::ComponentVtable::for_type::<$type>()
        }
    };
}

mod bounding_box;
mod dimension;
mod drawing_object;
mod layer;
mod name;
mod styles;
mod viewport;
mod vtable;

pub use bounding_box::BoundingBox;
pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable};
pub use styles::{LineStyle, PointStyle, WindowStyle};
pub use viewport::Viewport;
pub use vtable::ComponentVtable;

use specs::{Entity, World};
use std::any::Any;

/// Register all [`specs::Component`]s.
pub fn register(world: &mut World) {
    for vtable in inventory::iter::<ComponentVtable> {
        vtable.register(world);
    }
}

/// Looks up all [`Component`]s associated with this [`Entity`], yielding copies
/// of their current value.
///
/// # Note
///
/// This requires the [`decl_component!()`] macro to have been invoked on
/// the [`Component`].
pub fn attached_to_entity(
    world: &World,
    entity: Entity,
) -> impl Iterator<Item = (&'static ComponentVtable, Box<dyn Any>)> + '_ {
    inventory::iter::<ComponentVtable>
        .into_iter()
        .filter_map(move |vtable| match vtable.get_cloned(world, entity) {
            Some(got) => Some((vtable, got)),
            None => None,
        })
}
