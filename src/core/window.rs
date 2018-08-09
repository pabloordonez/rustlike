use core::drawing::point_2d::Point2d;
use core::drawing::size_2d::Size2d;
use core::Result;

pub trait Window {
    /// Gets the window client area size.
    fn get_window_client_size(&self) -> Result<Size2d>;

    /// Gets the window size.
    fn get_window_size(&self) -> Result<Size2d>;

    /// Sets the window size.
    fn set_window_size(&self, size: Size2d) -> Result<()>;

    /// Gets the window size.
    fn get_window_position(&self) -> Result<Point2d>;

    /// Sets the window size.
    fn set_window_position(&self, position: Point2d) -> Result<()>;
}
