use specs::{Component, World, WorldExt};
use std::any;

/// Functions for working with generic [`Component`]s without needing to drag a
/// type variable around.
#[derive(Copy, Clone)]
pub(crate) struct ComponentVtable {
    name: &'static str,
    register: fn(world: &mut World),
}

impl ComponentVtable {
    /// Create the [`ComponentVtable`] corresponding to a particular type.
    pub fn for_type<T>() -> Self
    where
        T: Component,
        T::Storage: Default,
    {
        ComponentVtable {
            name: any::type_name::<T>(),
            register: |world| {
                world.register::<T>();
            },
        }
    }

    /// A human-readable version of the [`Component`]'s name.
    pub fn name(&self) -> &'static str { self.name }

    /// Register this component with the [`World`].
    pub(crate) fn register(&self, world: &mut World) { (self.register)(world); }
}
