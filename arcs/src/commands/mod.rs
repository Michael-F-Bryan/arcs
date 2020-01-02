pub struct UndoRedoBuffer {}

pub trait CommandContext {}

/// An operation which can be executed against a target.
pub trait Command<T> {
    fn execute(&self, target: &mut T);
}

impl<F, T> Command<T> for F
where
    F: Fn(&mut T),
{
    fn execute(&self, target: &mut T) { self(target); }
}
