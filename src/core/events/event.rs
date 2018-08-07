use core::size_2d::Size2d;
use core::point_2d::Point2d;

/// Enumerates all the possible mouse event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum MouseEventType {
    MouseMove,
    MouseDown,
    MouseUp,
    Click,
    DoubleClick
}

/// Enumerates all the possible keyboard event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum KeyboardEventType {
    KeyDown,
    KeyUp,
    KeyPress
}

/// Enumerates all the possible window event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum WindowEventType {
    WindowMove,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowClose
}

/// Represents a mouse event like mouse move or mouse down.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub button_pressed: u8,
    pub position: Point2d
}


/// Represents a keyboard event like key down or key up.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct KeyboardEvent {
    pub event_type: KeyboardEventType,
    pub key_pressed: u16,
    pub character: char,
    pub control: bool,
    pub shift: bool,
    pub alt: bool
}

/// Represents a window event like window moved or window resized.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct WindowEvent {
    pub event_type: WindowEventType,
    pub position: Point2d,
    pub size: Size2d
}

/// Event object enumeration can be one of the valid event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Window(WindowEvent)
}