//! Core geometry primitives and algorithms extracted from the [`arcs`][arcs]
//! crate.
//!
//! [arcs]: https://crates.io/crates/arcs/

#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

pub mod algorithms;
mod bounding_box;
pub mod primitives;
mod types;

pub use bounding_box::BoundingBox;

/// An [`primitives::Arc`] in [`DrawingSpace`].
pub type Arc = primitives::Arc<DrawingSpace>;
/// A [`primitives::Line`] in [`DrawingSpace`].
pub type Line = primitives::Line<DrawingSpace>;

pub use types::{
    centre_of_three_points, Angle, CanvasSpace, DrawingSpace, Length,
    Orientation, Point, Transform, Vector,
};
