mod bounds;
mod name_table_bookkeeping;

pub use bounds::SyncBounds;
pub use name_table_bookkeeping::NameTableBookkeeping;

use specs::{DispatcherBuilder, World};

/// Register any necessary background tasks with a [`DispatcherBuilder`].
pub fn register_background_tasks<'a, 'b>(
    builder: DispatcherBuilder<'a, 'b>,
    world: &World,
) -> DispatcherBuilder<'a, 'b> {
    builder
        .with(
            NameTableBookkeeping::new(world),
            NameTableBookkeeping::NAME,
            &[],
        )
        .with(SyncBounds::new(world), SyncBounds::NAME, &[])
}
