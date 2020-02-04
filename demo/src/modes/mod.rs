mod add_arc_mode;
mod add_line_mode;
mod add_point_mode;
mod idle;

pub use add_arc_mode::AddArcMode;
pub use add_line_mode::AddLineMode;
pub use add_point_mode::AddPointMode;
pub use idle::Idle;

use arcs::{
    algorithms::Translate,
    components::{DrawingObject, Selected, Space, Viewport},
    CanvasSpace, DrawingSpace, Point, Vector,
};
use euclid::Point2D;
use genawaiter::sync::{Co, Gen};
use specs::prelude::*;
use std::{any::Any, fmt::Debug};

/// Contextual information passed to each [`State`] when it handles events.
pub trait ApplicationContext {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
    fn viewport(&self) -> Entity;

    /// An optimisation hint that the canvas doesn't need to be redrawn after
    /// this event handler returns.
    fn suppress_redraw(&mut self) {}

    /// Get a list of all the entities which lie "under" a point, for some
    /// definition of "under".
    ///
    /// Typically this will be implemented by the drawing canvas having some
    /// sort of "pick box" where anything within, say, 3 pixels of something is
    /// considered to be "under" it.
    fn entities_under_point<'this>(
        &'this self,
        location: Point,
    ) -> Box<dyn Iterator<Item = Entity> + 'this> {
        const PIXEL_RADIUS: f64 = 3.0;

        let viewports = self.world().read_storage::<Viewport>();

        let Viewport {
            pixels_per_drawing_unit,
            ..
        } = viewports.get(self.viewport()).unwrap();

        /// Generators aren't stable so we use the `genawaiter` hack to
        /// "close over" our `Space`.
        async fn iter(
            world: &World,
            pixels_per_drawing_unit: euclid::Scale<
                f64,
                DrawingSpace,
                CanvasSpace,
            >,
            location: Point,
            co: Co<Entity>,
        ) {
            let search_radius = pixels_per_drawing_unit.get() / PIXEL_RADIUS;

            let space = world.read_resource::<Space>();

            for spatial in space.query_point(location, search_radius) {
                co.yield_(spatial.entity).await;
            }
        }

        Box::new(
            Gen::new(|co| {
                iter(self.world(), *pixels_per_drawing_unit, location, co)
            })
            .into_iter(),
        )
    }

    /// Mark an object as being selected.
    fn select(&mut self, target: Entity) {
        self.world()
            .write_storage()
            .insert(target, Selected)
            .unwrap();
    }

    /// Clear the selection.
    fn unselect_all(&mut self) {
        self.world().write_storage::<Selected>().clear();
    }

    /// Translate all selected objects by a specific amount.
    fn translate_selection(&mut self, displacement: Vector) {
        let world = self.world();
        let (entities, selected, mut drawing_objects): (
            Entities,
            ReadStorage<Selected>,
            WriteStorage<DrawingObject>,
        ) = world.system_data();

        for (_, _, drawing_object) in
            (&entities, &selected, &mut drawing_objects).join()
        {
            drawing_object.geometry.translate(displacement);
        }
    }
}

impl<'a, A: ApplicationContext + ?Sized> ApplicationContext for &'a mut A {
    fn world(&self) -> &World { (**self).world() }

    fn world_mut(&mut self) -> &mut World { (**self).world_mut() }

    fn viewport(&self) -> Entity { (**self).viewport() }

    fn suppress_redraw(&mut self) { (**self).suppress_redraw(); }
}

pub trait State: Debug + AsAny {
    /// The [`State`] has been cancelled and needs to clean up any temporary
    /// objects it created.
    fn on_cancelled(&mut self, _ctx: &mut dyn ApplicationContext) {}

    /// The left mouse button was pressed.
    fn on_mouse_down(
        &mut self,
        _ctx: &mut dyn ApplicationContext,
        _event_args: &MouseEventArgs,
    ) -> Transition {
        Transition::DoNothing
    }

    /// The left mouse button was released.
    fn on_mouse_up(
        &mut self,
        _ctx: &mut dyn ApplicationContext,
        _event_args: &MouseEventArgs,
    ) -> Transition {
        Transition::DoNothing
    }

    /// The mouse moved.
    fn on_mouse_move(
        &mut self,
        ctx: &mut dyn ApplicationContext,
        _event_args: &MouseEventArgs,
    ) -> Transition {
        ctx.suppress_redraw();
        Transition::DoNothing
    }

    /// A button was pressed on the keyboard.
    fn on_key_pressed(
        &mut self,
        _ctx: &mut dyn ApplicationContext,
        _event_args: &KeyboardEventArgs,
    ) -> Transition {
        Transition::DoNothing
    }
}

/// A helper trait for casting `self` to [`Any`].
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<A: Any> AsAny for A {
    fn as_any(&self) -> &dyn Any { self }
}

/// Instructions to the state machine returned by the various event handlers
/// in [`State`].
#[derive(Debug)]
pub enum Transition {
    ChangeState(Box<dyn State>),
    DoNothing,
}

impl Transition {
    /// Checks whether the transition will change to a particular [`State`].
    pub fn changes_to<S>(&self) -> bool
    where
        S: State + 'static,
    {
        match self {
            Transition::ChangeState(new_state) => {
                (**new_state).as_any().is::<S>()
            },
            _ => false,
        }
    }

    /// Is this a no-op [`Transition`]?
    pub fn does_nothing(&self) -> bool {
        match self {
            Transition::DoNothing => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MouseEventArgs {
    /// The mouse's location on the drawing.
    pub location: Point2D<f64, DrawingSpace>,
    /// The mouse's location on the canvas.
    pub cursor: Point2D<f64, CanvasSpace>,
    /// The state of the mouse buttons.
    pub button_state: MouseButtons,
}

bitflags::bitflags! {
    /// Which mouse button (or buttons) are pressed?
    pub struct MouseButtons: u8 {
        const LEFT_BUTTON = 0;
        const RIGHT_BUTTON = 1;
        const MIDDLE_BUTTON = 2;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyboardEventArgs {
    pub shift_pressed: bool,
    pub control_pressed: bool,
    /// The semantic meaning of the key currently being pressed, if there is
    /// one.
    pub key: Option<VirtualKeyCode>,
}

impl KeyboardEventArgs {
    /// Create a new [`KeyboardEventArgs`] which just presses a key.
    pub fn pressing(key: VirtualKeyCode) -> Self {
        KeyboardEventArgs {
            key: Some(key),
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VirtualKeyCode {
    Escape,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
}
