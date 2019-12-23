//! Core algorithms and graphical primitives for the `arcs` CAD library.

pub mod algorithms;
pub mod components;
pub mod primitives;
pub mod render;
mod vector;

pub use vector::{Orientation, Vector};
