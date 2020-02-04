//! Common components used by the `arcs` CAD library.

mod bounding_box;
mod dimension;
mod drawing_object;
mod layer;
mod name;
mod styles;
mod viewport;
mod vtable;
mod spatial_entity;

pub use bounding_box::BoundingBox;
pub use dimension::Dimension;
pub use drawing_object::{DrawingObject, Geometry};
pub use layer::Layer;
pub use name::{Name, NameTable};
pub use styles::{LineStyle, PointStyle, WindowStyle};
pub use viewport::Viewport;
pub(crate) use vtable::ComponentVtable;
pub use spatial_entity::{SpatialEntity, Space};

use specs::World;

/// Get an iterator over the [`ComponentVtable`] for all known
/// [`specs::Component`] types.
pub(crate) fn known_components(
) -> impl Iterator<Item = &'static ComponentVtable> + 'static {
    lazy_static::lazy_static! {
        static ref VTABLES: Vec<ComponentVtable> = vec![
            ComponentVtable::for_type::<BoundingBox>(),
            ComponentVtable::for_type::<DrawingObject>(),
            ComponentVtable::for_type::<Layer>(),
            ComponentVtable::for_type::<Name>(),
            ComponentVtable::for_type::<LineStyle>(),
            ComponentVtable::for_type::<PointStyle>(),
            ComponentVtable::for_type::<WindowStyle>(),
            ComponentVtable::for_type::<Viewport>(),
        ];
    }

    VTABLES.iter()
}

/// Register all [`specs::Component`]s.
pub fn register(world: &mut World) {
    log::debug!("Registering all components");

    for component in known_components() {
        log::debug!("Registering {}", component.name());
        component.register(world);
    }
}
