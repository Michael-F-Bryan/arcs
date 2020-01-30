//! A Rust CAD System - A library for building 2D *Computer Aided Design*
//! applications.

#![forbid(unsafe_code)]

pub mod algorithms;
pub mod components;
pub mod primitives;
pub mod systems;
mod vector;
pub mod window;

pub use vector::{
    centre_of_three_points, CanvasSpace, DrawingSpace, Orientation, Transform,
    Vector,
};
