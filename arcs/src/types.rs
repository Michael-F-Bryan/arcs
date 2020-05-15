use euclid::{Point2D, Vector2D};

/// The cartesian coordinate system used by everything in a drawing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum DrawingSpace {}

/// The coordinate system used for graphical objects rendered to a canvas.
///
/// The convention is for the canvas/window's top-left corner to be the origin,
/// with positive `x` going to the right and positive `y` going down the screen.
///
/// To convert from [`DrawingSpace`] to [`CanvasSpace`] you'll need a
/// [`crate::components::Viewport`] representing the area on the drawing the
/// canvas will display. The [`crate::window`] module exposes various utility
/// functions for converting back and forth, with
/// [`crate::window::to_drawing_coordinates()`] and
/// [`crate::window::to_canvas_coordinates()`] being the most useful.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum CanvasSpace {}

/// A 2D vector for working in [`DrawingSpace`].
pub type Vector = Vector2D<f64, DrawingSpace>;
/// A transform matrix which for translating something within [`DrawingSpace`].
pub type Transform = euclid::Transform2D<f64, DrawingSpace, DrawingSpace>;
/// A location in [`DrawingSpace`].
pub type Point = Point2D<f64, DrawingSpace>;
/// A length in [`DrawingSpace`].
pub type Length = euclid::Length<f64, DrawingSpace>;
