//! A Rust CAD System - A library for building 2D *Computer Aided Design*
//! applications.

#![forbid(unsafe_code)]

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
