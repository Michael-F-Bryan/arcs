use arcs_render::{px, Point};
use yew::{
    events::IMouseEvent,
};


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Msg {
    CanvasClicked(MouseEvent),
    /// The page has been loaded.
    CanvasLoaded,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseEvent {
    pub client: Point,
    pub screen: Point,
    pub movement: Point,
}

impl<E: IMouseEvent> From<E> for MouseEvent {
    fn from(event: E) -> MouseEvent {
        MouseEvent {
            client: Point::new(px(event.client_x()), px(event.client_y())),
            screen: Point::new(px(event.screen_x()), px(event.screen_y())),
            movement: Point::new(
                px(event.movement_x()),
                px(event.movement_y()),
            ),
        }
    }
}