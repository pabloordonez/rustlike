use super::point_2d::Point2d;

pub trait Mouse {
    /// Gets the absolute mouse position.
    fn get_absolute_position(&self) -> Point2d;

    /// Gets the mouse position in relation to the client window.
    fn get_client_position(&self) -> Point2d;

    /// Sets the mouse position.
    fn set_position(&self, position: Point2d);

    /// Shows or hides the mouse cursor.
    fn show_cursor(&self, visible: bool);
}
