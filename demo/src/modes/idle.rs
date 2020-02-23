use crate::modes::{
    AddArcMode, AddLineMode, AddPointMode, ApplicationContext,
    KeyboardEventArgs, MouseEventArgs, State, Transition, VirtualKeyCode,
};
use arcs::Point;

#[derive(Debug)]
pub struct Idle {
    nested: Box<dyn State>,
}

impl State for Idle {
    fn on_key_pressed(
        &mut self,
        _ctx: &mut dyn ApplicationContext,
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

    fn on_mouse_down(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        args: &MouseEventArgs,
    ) -> Transition {
        if let Transition::ChangeState(new_state) =
            self.nested.on_mouse_down(ctx, args)
        {
            self.nested = new_state;
        }

        Transition::DoNothing
    }

    fn on_mouse_up(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        args: &MouseEventArgs,
    ) -> Transition {
        if let Transition::ChangeState(new_state) =
            self.nested.on_mouse_up(ctx, args)
        {
            self.nested = new_state;
        }

        Transition::DoNothing
    }

    fn on_mouse_move(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        _event_args: &MouseEventArgs,
    ) -> Transition {
        ctx.suppress_redraw();
        Transition::DoNothing
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
        ctx: &mut dyn ApplicationContext,
        args: &MouseEventArgs,
    ) -> Transition {
        let first_item_under_cursor =
            ctx.entities_under_point(args.location).next();

        match first_item_under_cursor {
            Some(entity) => {
                ctx.select(entity);
                Transition::ChangeState(Box::new(DraggingSelection::from_args(
                    args,
                )))
            },
            _ => {
                ctx.unselect_all();
                Transition::DoNothing
            },
        }
    }

    fn on_mouse_move(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        _event_args: &MouseEventArgs,
    ) -> Transition {
        ctx.suppress_redraw();
        Transition::DoNothing
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
        DraggingSelection {
            previous_location: args.location,
        }
    }
}

impl State for DraggingSelection {
    fn on_mouse_move(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        args: &MouseEventArgs,
    ) -> Transition {
        ctx.translate_selection(args.location - self.previous_location);
        self.previous_location = args.location;

        Transition::DoNothing
    }

    fn on_mouse_up(
        &mut self,
        _ctx: &mut dyn ApplicationContext,
        _args: &MouseEventArgs,
    ) -> Transition {
        Transition::ChangeState(Box::new(WaitingToSelect::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcs::{
        components::{Layer, Name, Viewport},
        Point,
    };
    use euclid::Scale;
    use specs::{Builder, Entity, World, WorldExt};

    struct DummyContext {
        world: World,
        viewport: Entity,
        default_layer: Entity,
    }

    impl Default for DummyContext {
        fn default() -> Self {
            let mut world = World::new();
            arcs::components::register(&mut world);
            let viewport = world
                .create_entity()
                .with(Viewport {
                    centre: Point::zero(),
                    pixels_per_drawing_unit: Scale::new(1.0),
                })
                .build();

            let default_layer = Layer::create(
                world.create_entity(),
                Name::from("default"),
                Layer::default(),
            );

            DummyContext {
                world,
                viewport,
                default_layer,
            }
        }
    }

    impl ApplicationContext for DummyContext {
        fn world(&self) -> &World { &self.world }

        fn world_mut(&mut self) -> &mut World { &mut self.world }

        fn viewport(&self) -> Entity { self.viewport }

        fn default_layer(&self) -> Entity { self.default_layer }
    }

    #[test]
    fn change_to_arc_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyContext::default();
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::A);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddArcMode>());
    }

    #[test]
    fn change_to_line_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyContext::default();
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::L);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddLineMode>());
    }

    #[test]
    fn change_to_point_mode() {
        let mut idle = Idle::default();
        let mut drawing = DummyContext::default();
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::P);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.changes_to::<AddPointMode>());
    }

    #[test]
    fn pressing_any_other_key_does_nothing() {
        let mut idle = Idle::default();
        let mut drawing = DummyContext::default();
        let args = KeyboardEventArgs::pressing(VirtualKeyCode::Q);

        let got = idle.on_key_pressed(&mut drawing, &args);

        assert!(got.does_nothing());
    }
}
