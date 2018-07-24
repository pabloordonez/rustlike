use super::point_2d::Point2d;
use super::size_2d::Size2d;

pub trait Window {
    /// Gets the window client area size.
    fn get_window_client_size(&self) -> Size2d;

    /// Gets the window size.
    fn get_window_size(&self) -> Size2d;

    /// Sets the window size.
    fn set_window_size(&self, size: Size2d);

    /// Gets the window size.
    fn get_window_position(&self) -> Point2d;

    /// Sets the window size.
    fn set_window_position(&self, position: Point2d);
}
