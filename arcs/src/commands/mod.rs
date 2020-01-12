//! Transactional updates which can be applied to the world for a basic
//! undo/redo mechanism.

mod changes;

pub use changes::{Builder, ChangeRecorder};

pub struct UndoRedoBuffer {}

pub trait Command {
    fn execute(&self, world: &mut ChangeRecorder<'_>);
}
