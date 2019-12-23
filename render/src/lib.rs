//! Rendering and window management for the `arcs` CAD library.

pub use piet;

mod viewport;

pub use viewport::Viewport;

use arcs_core::{components::Visual, algorithms::BoundingBox};
use piet::{Error, RenderContext};
use specs::{World, WorldExt};

/// Register all [`specs::Component`]s used by the rendering system.
pub fn setup(world: &mut World) {
    world.register::<Viewport>();
    world.register::<Visual>();
}

/// Render the drawing using a particular backend.
pub fn render<R>(
    _backend: &mut R,
    _world: &World,
    _window_size: BoundingBox,
    _viewport: &Viewport,
) -> Result<(), Error>
where
    R: RenderContext,
{
    unimplemented!()
}
