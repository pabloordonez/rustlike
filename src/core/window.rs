use super::point_2d::Point2d;
use super::size_2d::Size2d;

pub trait Window {
    /// Gets the window client area size.
    fn get_window_client_size(&self) ->  Result<Size2d, &'static str>;

    /// Gets the window size.
    fn get_window_size(&self) ->  Result<Size2d, &'static str>;

    /// Sets the window size.
    fn set_window_size(&self, size: Size2d) -> Result<(), &'static str>;

    /// Gets the window size.
    fn get_window_position(&self) ->  Result<Point2d, &'static str>;

    /// Sets the window size.
    fn set_window_position(&self, position: Point2d) -> Result<(), &'static str>;
}