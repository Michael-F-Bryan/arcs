//! Basic geometric types which are generic over their coordinate space.

mod arc;
pub mod interpolated_spline;
mod line;

pub use arc::Arc;
pub use line::Line;
