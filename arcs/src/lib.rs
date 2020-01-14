//! A Rust CAD System - A library for building 2D *Computer Aided Design*
//! applications.

#![forbid(unsafe_code)]

pub mod algorithms;
pub mod commands;
pub mod components;
pub mod primitives;
pub mod systems;
mod vector;
mod transform;
pub mod window;

pub use vector::{Orientation, Vector};
