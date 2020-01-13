//! Transactional updates which can be applied to the world for a basic
//! undo/redo mechanism.

mod changes;
mod undo_redo_buffer;

pub use changes::{Builder, ChangeRecorder, ChangeSet};
pub use undo_redo_buffer::{UndoRedoBuffer, UndoRedoError};

pub trait Command {
    fn execute(&self, world: &mut ChangeRecorder<'_>);
}
