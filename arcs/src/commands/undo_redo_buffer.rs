use crate::commands::{ChangeRecorder, ChangeSet, Command};
use specs::World;

#[derive(Default)]
pub struct UndoRedoBuffer {
    changes: Vec<ChangeSet>,
    cursor: usize,
}

impl UndoRedoBuffer {
    pub fn new() -> UndoRedoBuffer { UndoRedoBuffer::default() }

    pub fn execute<C: Command>(&mut self, command: C, world: &World) {
        let mut recorder = ChangeRecorder::new(world);
        command.execute(&mut recorder);

        self.changes.push(recorder.into_changes());
    }

    pub fn can_undo(&self) -> bool { self.cursor > 0 }

    pub fn can_redo(&self) -> bool { self.cursor <= self.changes.len() }

    pub fn undo(&mut self, world: &World) -> Result<(), UndoRedoError> {
        if !self.can_undo() {
            return Err(UndoRedoError);
        }

        self.changes[self.cursor - 1].revert(world);
        self.cursor -= 1;

        Ok(())
    }

    pub fn redo(&mut self, world: &World) -> Result<(), UndoRedoError> {
        if !self.can_redo() {
            return Err(UndoRedoError);
        }

        self.changes[self.cursor].apply(world);
        self.cursor += 1;

        Ok(())
    }
}

pub struct UndoRedoError;
