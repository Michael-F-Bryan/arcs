//! Common algorithms.

mod approximate;
mod bounding_box;
mod closest_point;
mod length;
mod translate;
mod scale;
mod scale_non_uniform;

pub use approximate::{Approximate, ApproximatedArc};
pub use bounding_box::Bounded;
pub use closest_point::{Closest, ClosestPoint};
pub use length::Length;
pub use translate::Translate;
pub use scale::Scale;
pub use scale_non_uniform::ScaleNonUniform;