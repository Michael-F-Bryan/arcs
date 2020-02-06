use specs::prelude::*;
use specs_derive::Component;

/// An empty [`Component`] used to mark an [`Entity`] as selected.
#[derive(Debug, Copy, Clone, Default, PartialEq, Component)]
#[storage(NullStorage)]
pub struct Selected;
