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

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> (usize, usize);

    /// Gets the window size.
    fn get_window_size(&self) -> (usize, usize);

    /// Sets the window size.
    fn set_window_size(&self, width: usize, height: usize);

    /// Gets the window size.
    fn get_window_position(&self) -> (usize, usize);

    /// Sets the window size.
    fn set_window_position(&self, x: usize, y: usize);

    /// Clears the console screen.
    fn clear(&self);

    /// Draws a `CellBuffer` to the screen.
    fn draw(&self, cell_buffer: &CellBuffer);
}