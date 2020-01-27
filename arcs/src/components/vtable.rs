use specs::{Component, Entity, World, WorldExt};
use std::{
    any::{self, Any, TypeId},
    fmt::{self, Debug, Formatter},
};

/// A vtable which gives the engine reflection-like abilities.
#[derive(Copy, Clone)]
pub(crate) struct ComponentVtable {
    type_id: TypeId,
    name: &'static str,
    register: fn(world: &mut World),
    get_cloned: fn(world: &World, entity: Entity) -> Option<Box<dyn Any>>,
    debug: fn(item: &dyn Any, f: &mut Formatter<'_>) -> fmt::Result,
}

impl ComponentVtable {
    /// Create the [`ComponentVtable`] corresponding to a particular type.
    pub fn for_type<T>() -> Self
    where
        T: Component + Clone + Debug,
        <T as Component>::Storage: Default,
    {
        ComponentVtable {
            type_id: TypeId::of::<T>(),
            name: any::type_name::<T>(),
            register: |world| {
                world.register::<T>();
            },
            get_cloned: |world, entity| {
                world
                    .read_storage::<T>()
                    .get(entity)
                    .cloned()
                    .map(|item| Box::new(item) as Box<dyn Any>)
            },
            debug: |item, f| match item.downcast_ref::<T>() {
                Some(item) => Debug::fmt(item, f),
                None => panic!("Expected a {}", any::type_name::<T>()),
            },
        }
    }

    pub fn applies_to<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    /// A human-readable version of the [`Component`]'s name.
    pub fn name(&self) -> &'static str { self.name }

    /// Get the [`Debug`] representation of this type.
    pub fn debug(&self, item: &dyn Any, f: &mut Formatter<'_>) -> fmt::Result {
        (self.debug)(item, f)
    }

    /// Register this component with the [`World`].
    pub(crate) fn register(&self, world: &mut World) { (self.register)(world); }

    /// Lookup the component associated with an entity, returning a copy if
    /// anything is found.
    pub(crate) fn get_cloned(
        &self,
        world: &World,
        entity: Entity,
    ) -> Option<Box<dyn Any>> {
        (self.get_cloned)(world, entity)
    }
}
