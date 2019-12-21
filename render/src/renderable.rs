use crate::Canvas;

/// Something that can be rendered to a [`Canvas`].
pub trait Renderable {
    fn render<C>(&self, canvas: C)
    where
        C: Canvas;
}
