//! Core geometry primitives and algorithms extracted from the [`arcs`][arcs]
//! crate.
//!
//! [arcs]: https://crates.io/crates/arcs/

#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    intra_doc_link_resolution_failure
)]

pub mod algorithms;
mod bounding_box;
mod orientation;
pub mod primitives;

pub use bounding_box::BoundingBox;
pub use orientation::{centre_of_three_points, Orientation};

/// A strongly-typed angle, useful for dealing with the pesky modular arithmetic
/// normally associated with circles and angles.
pub type Angle = euclid::Angle<f64>;
