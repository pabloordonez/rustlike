use core::drawing::cell_buffer::CellBuffer;
use core::drawing::point_2d::Point2d;
use core::drawing::size_2d::Size2d;
use core::window::Window;
use core::Result;

pub trait Terminal {
    /// Disposes the terminal object-
    fn dispose(&self) -> Result<()>;

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&self, visible: bool) -> Result<()>;

    /// Moves the console cursor to a given position.
    fn set_cursor(&self, positon: Point2d) -> Result<()>;

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Result<Size2d>;

    /// Gets the character size in pixel units.
    fn get_char_size(&self, window: &Window) -> Result<Size2d>;

    /// Clears the console screen.
    fn clear(&self) -> Result<()>;

    /// Draws a `CellBuffer` to the screen.
    fn write(&self, cell_buffer: &CellBuffer) -> Result<()>;
}
