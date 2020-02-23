//! Common algorithms.

mod affine_transform;
mod approximate;
mod bounding_box;
mod closest_point;
mod length;
mod line_simplification;
mod scale;
mod scale_non_uniform;
mod translate;

pub use affine_transform::AffineTransformable;
pub use approximate::{Approximate, ApproximatedArc};
pub use bounding_box::Bounded;
pub use closest_point::{Closest, ClosestPoint};
pub use length::Length;
pub use line_simplification::simplify;
pub use scale::Scale;
pub use scale_non_uniform::ScaleNonUniform;
pub use translate::Translate;
