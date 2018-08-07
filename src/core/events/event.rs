use core::size_2d::Size2d;
use core::point_2d::Point2d;

/// Enumerates all the possible mouse event types.
#[allow(dead_code)]
pub enum MouseEventType {
    MouseMove,
    MouseDown,
    MouseUp,
    Click,
    DoubleClick
}

/// Enumerates all the possible keyboard event types.
#[allow(dead_code)]
pub enum KeyboardEventType {
    KeyDown,
    KeyUp,
    KeyPress
}

/// Enumerates all the possible window event types.
#[allow(dead_code)]
pub enum WindowEventType {
    WindowMove,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowClose
}

/// Represents a mouse event like mouse move or mouse down.
#[allow(dead_code)]
pub struct MouseEvent {
    event_type: MouseEventType,
    button_pressed: u8,
    position: Point2d
}


/// Represents a keyboard event like key down or key up.
#[allow(dead_code)]
pub struct KeyboardEvent {
    event_type: WindowEventType,
    key_pressed: u16,
    character: char,
    control: bool,
    shift: bool,
    alt: bool
}

/// Represents a window event like window moved or window resized.
#[allow(dead_code)]
pub struct WindowEvent {
    event_type: WindowEventType,
    position: Point2d,
    size: Size2d
}

/// Event object enumeration can be one of the valid event types.
#[allow(dead_code)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Window(WindowEvent)
}