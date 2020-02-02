use crate::modes::{
    AddArcMode, AddLineMode, AddPointMode, Drawing, KeyboardEventArgs, State,
    Transition, VirtualKeyCode,
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

impl State for WaitingToSelect {}

#[cfg(test)]
mod tests {
    use super::*;
    use arcs::{components::DrawingObject, Point};
    use specs::Entity;

    struct DummyDrawing;

    impl Drawing for DummyDrawing {
        fn entities_under_point(
            &self,
            _location: Point,
        ) -> Box<dyn Iterator<Item = (Entity, &DrawingObject)>> {
            unimplemented!()
        }
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
