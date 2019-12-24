//! Rendering and window management for the `arcs` CAD library.
//!
//! # A Note on Conventions
//!
//! The rendering system needs to work with two coordinate systems at the same
//! time. To avoid confusion,
//!
//! > **In *Drawing Space* we'll use [`crate::Vector`] and types from
//! > [`crate::primitives`], and when in *Canvas Space* we'll use types from
//! > the [`kurbo`] crate**

mod renderer;

pub use renderer::Renderer;

use crate::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// The location (in drawing units) this viewport is centred on.
    pub centre: Vector,
    /// The number of pixels each drawing unit should take up on the screen.
    pub pixels_per_drawing_unit: f64,
}
