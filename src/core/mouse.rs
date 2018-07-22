pub trait Mouse {

    /// Creates a new instance of the mouse.
    fn new() -> Self;

    /// Gets the absolute mouse position.
    fn get_absolute_position(&self) -> (usize, usize);

    /// Gets the mouse position in relation to the client window.
    fn get_client_position(&self) -> (usize, usize);

    /// Sets the mouse position.
    fn set_position(&self, x: usize, y: usize);

    /// Shows or hides the mouse cursor.
    fn show_cursor(&self, visible: bool);
}
