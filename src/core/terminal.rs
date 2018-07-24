use core::cell_buffer::CellBuffer;
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use core::window::Window;

pub trait Terminal {
    /// Disposes the terminal object-
    fn dispose(&self);

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&self, visible: bool);

    /// Moves the console cursor to a given position.
    fn set_cursor(&self, positon: Point2d);

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Size2d;

    /// Gets the character size in pixel units.
    fn get_char_size(&self, window: &Window) -> Size2d;

    /// Clears the console screen.
    fn clear(&self);

    /// Draws a `CellBuffer` to the screen.
    fn write(&self, cell_buffer: &CellBuffer);
}
