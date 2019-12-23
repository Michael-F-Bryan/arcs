use kurbo::Point;
use yew::events::IMouseEvent;

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
            client: Point::new(
                event.client_x().into(),
                event.client_y().into(),
            ),
            screen: Point::new(
                event.screen_x().into(),
                event.screen_y().into(),
            ),
            movement: Point::new(
                event.movement_x().into(),
                event.movement_y().into(),
            ),
        }
    }
}
