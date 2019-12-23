//! Rendering and window management for the `arcs` CAD library.

pub extern crate piet;

mod viewport;

pub use viewport::Viewport;

use arcs_core::algorithms::BoundingBox;
use piet::{Error, RenderContext};
use specs::World;

pub fn render<R>(
    world: &World,
    window_size: BoundingBox,
    viewport: &Viewport,
    backend: &mut R,
) -> Result<(), Error>
where
    R: RenderContext,
{
    unimplemented!()
}
