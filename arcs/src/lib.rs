//! A Rust CAD System - A library for building 2D *Computer Aided Design*
//! applications.
//!
//! ## A Note on Conventions
//!
//! When using this crate you'll frequently need to work with multiple
//! coordinate spaces. To prevent accidentally mixing up vectors or points in
//! different coordinate spaces (see [the Mars Climate Orbiter][mco]), we use
//! [`euclid`]'s ability to "tag" a geometry primitive with something
//! representing the coordinate space it belongs to.
//!
//! For convenience we expose type aliases for the main coordinate space you'll
//! be using, [`DrawingSpace`].
//!
//! For more details on when each coordinate space is used, consult the docs for
//! [`DrawingSpace`] and [`CanvasSpace`].
//!
//! [mco]: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter

#![forbid(unsafe_code)]
#[deny(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]
pub mod algorithms;
mod arc;
pub mod components;
mod line;
pub mod systems;
mod types;
pub mod window;

pub use arc::Arc;
pub use line::Line;

pub use types::{
    centre_of_three_points, Angle, CanvasSpace, DrawingSpace, Length,
    Orientation, Point, Transform, Vector,
};
