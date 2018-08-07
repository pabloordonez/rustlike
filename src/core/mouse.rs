use core::point_2d::Point2d;
use core::Result;

const LEFT_BUTTON: u8 = 0b0001;
const MIDDLE_BUTTON: u8 = 0b0010;
const RIGHT_BUTTON: u8 = 0b0100;

pub trait Mouse {
    /// Gets the absolute mouse position.
    fn get_absolute_position(&self) -> Result<Point2d>;

    /// Gets the mouse position in relation to the client window.
    fn get_client_position(&self) -> Result<Point2d>;

    /// Sets the mouse position.
    fn set_position(&self, position: Point2d) -> Result<()>;

    /// Shows or hides the mouse cursor.
    fn show_cursor(&self, visible: bool) -> Result<()>;
}

/// Indicates if a mouse button is pressed.
#[inline]
#[allow(dead_code)]
pub fn is_button_pressed(event_button: u8, button: u8) -> bool {
    event_button & button != 0
}

/// Indicates if the left mouse button is pressed.
#[inline]
#[allow(dead_code)]
pub fn is_left_button_pressed(event_button: u8) -> bool {
    is_button_pressed(event_button, LEFT_BUTTON)
}

/// Indicates if the middle mouse button is pressed.
#[inline]
#[allow(dead_code)]
pub fn is_middle_button_pressed(event_button: u8) -> bool {
    is_button_pressed(event_button, MIDDLE_BUTTON)
}

/// Indicates if the right mouse button is pressed.
#[inline]
#[allow(dead_code)]
pub fn is_right_button_pressed(event_button: u8) -> bool {
    is_button_pressed(event_button, RIGHT_BUTTON)
}
