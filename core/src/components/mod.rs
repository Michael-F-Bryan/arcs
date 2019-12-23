mod layer;
pub mod name;

pub use layer::Layer;
pub use name::Name;

use name::NameTableBookkeeping;
use specs::{DispatcherBuilder, World};

/// Register any necessary background tasks with a [`DispatcherBuilder`].
pub fn register_background_tasks<'a, 'b>(
    builder: DispatcherBuilder<'a, 'b>,
    world: &World,
) -> DispatcherBuilder<'a, 'b> {
    builder.with(
        NameTableBookkeeping::new(world),
        NameTableBookkeeping::NAME,
        &[],
    )
}
