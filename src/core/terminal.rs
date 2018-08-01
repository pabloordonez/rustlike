use core::cell::Cell;
use core::cell_buffer::CellBuffer;
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use core::window::Window;

pub trait Terminal<TCell>
where
    TCell: Cell,
{
    // Creates a new cell buffer.
    fn create_buffer(size: Size2d) -> CellBuffer<TCell>;

    /// Disposes the terminal object-
    fn dispose(&self) -> Result<(), &'static str>;

    /// Shows or hides the cursor.
    fn set_cursor_visibility(&self, visible: bool) -> Result<(), &'static str>;

    /// Moves the console cursor to a given position.
    fn set_cursor(&self, positon: Point2d) -> Result<(), &'static str>;

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Result<Size2d, &'static str>;

    /// Gets the character size in pixel units.
    fn get_char_size(&self, window: &Window) -> Result<Size2d, &'static str>;

    /// Clears the console screen.
    fn clear(&self) -> Result<(), &'static str>;

    /// Draws a `CellBuffer` to the screen.
    fn write(&self, cell_buffer: &CellBuffer<TCell>) -> Result<(), &'static str>;
}
