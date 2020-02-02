use arcs::Point;
use crate::modes::{
    AddArcMode, AddLineMode, AddPointMode, Drawing, KeyboardEventArgs,
    MouseEventArgs, State, Transition, VirtualKeyCode,
};

#[derive(Debug)]
pub struct Idle {
    nested: Box<dyn State>,
}

impl State for Idle {
    fn on_key_pressed(
        &mut self,
        _drawing: &mut dyn Drawing,
        event_args: &KeyboardEventArgs,
    ) -> Transition {
        match event_args.key {
            Some(VirtualKeyCode::A) => {
                Transition::ChangeState(Box::new(AddArcMode::default()))
            },
            Some(VirtualKeyCode::P) => {
                Transition::ChangeState(Box::new(AddPointMode::default()))
            },
            Some(VirtualKeyCode::L) => {
                Transition::ChangeState(Box::new(AddLineMode::default()))
            },
            _ => Transition::DoNothing,
        }
    }
}

impl Default for Idle {
    fn default() -> Idle {
        Idle {
            nested: Box::new(WaitingToSelect),
        }
    }
}

/// [`Idle`]'s base sub-state.
///
/// We are waiting for the user to click so we can change the selection or start
/// dragging.
#[derive(Debug, Default)]
struct WaitingToSelect;

impl State for WaitingToSelect {
    fn on_mouse_down(
        &mut self,
        drawing: &mut dyn Drawing,
        args: &MouseEventArgs,
    ) -> Transition {
        let first_item_under_cursor =
            drawing.entities_under_point(args.location).next();

        match first_item_under_cursor {
            Some((entity, _)) => {
                drawing.select(entity);
                Transition::ChangeState(Box::new(DraggingSelection::from_args(args)))
            },
            _ => {
                drawing.unselect_all();
                Transition::DoNothing
            },
        }
    }
}

/// The left mouse button is currently pressed and the user is dragging items
/// around.
#[derive(Debug)]
struct DraggingSelection {
    previous_location: Point,
}

impl DraggingSelection {
    fn from_args(args: &MouseEventArgs) -> Self {
        DraggingSelection{
            previous_location: args.location,
        }
    }
}

impl State for DraggingSelection {
    fn on_mouse_move(
        &mut self,
        drawing: &mut dyn Drawing,
        args: &MouseEventArgs,
    ) -> Transition {
        drawing.translate_selection(args.location - self.previous_location);
        self.previous_location = args.location;

        Transition::DoNothing
    }

    fn on_mouse_up(
        &mut self,
        _drawing: &mut dyn Drawing,
        _args: &MouseEventArgs,
    ) -> Transition {
        Transition::ChangeState(Box::new(WaitingToSelect::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcs::{components::DrawingObject, Point, Vector};
    use specs::Entity;

    struct DummyDrawing;

    impl Drawing for DummyDrawing {
        fn entities_under_point(
            &self,
            _location: Point,
        ) -> Box<dyn Iterator<Item = (Entity, &DrawingObject)>> {
            unimplemented!()
        }

        fn select(&mut self, _target: Entity) { unimplemented!() }

        fn unselect_all(&mut self) { unimplemented!() }

        fn translate_selection(&mut self, _: Vector) { unimplemented!() }
    }

    #[test]
    fn change_to_arc_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyDrawing;
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::A);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddArcMode>());
    }

    #[test]
    fn change_to_line_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyDrawing;
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::L);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddLineMode>());
    }

    #[test]
    fn change_to_point_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyDrawing;
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::P);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddPointMode>());
    }

    #[test]
    fn pressing_any_other_key_does_nothing() {
        let mut idle = Idle::default();
        let mut drawing = DummyDrawing;
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::Q);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.does_nothing());
    }
}
