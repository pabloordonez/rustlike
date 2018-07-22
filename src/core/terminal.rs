use core::cellbuffer::CellBuffer;

pub trait Terminal {

    /// Creates a new terminal.
    fn new() -> Self;

    /// Disposes the terminal object-
    fn dispose(&self);

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&self, visible: bool);

    /// Moves the console cursor to a given position.
    fn set_cursor(&self, x: u16, y: u16);

    /// Gets the current console size.
    fn get_size(&self) -> (usize, usize);

    /// Sets the console size.
    fn set_size(&self, width: usize, height: usize);

    /// Clears the console screen.
    fn clear(&self);

    /// Draws a `CellBuffer` to the screen.
    fn draw(&self, cell_buffer: &CellBuffer);
}