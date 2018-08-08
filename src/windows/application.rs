use core::application::Application;
use core::events::device_state::KeyboardState;
use core::events::device_state::MouseState;
use core::events::event::Event;
use core::events::event::{
    KeyboardEvent, KeyboardEventType, MouseEvent, MouseEventType, WindowEvent, WindowEventType,
};
use core::events::event_queue::EventQueue;
use core::mouse::Mouse;
use core::point_2d::Point2d;
use core::terminal::Terminal;
use core::window::Window;
use core::Result;
use std::char::from_u32;
use windows::mouse::WindowsMouse;
use windows::terminal::WindowsTerminal;
use windows::winapi::um::consoleapi::{
    GetNumberOfConsoleInputEvents, ReadConsoleInputW, SetConsoleMode,
};
use windows::winapi::um::wincon::LEFT_ALT_PRESSED;
use windows::winapi::um::wincon::LEFT_CTRL_PRESSED;
use windows::winapi::um::wincon::RIGHT_ALT_PRESSED;
use windows::winapi::um::wincon::RIGHT_CTRL_PRESSED;
use windows::winapi::um::wincon::{
    FROM_LEFT_1ST_BUTTON_PRESSED, FROM_LEFT_2ND_BUTTON_PRESSED, FROM_LEFT_3RD_BUTTON_PRESSED,
    FROM_LEFT_4TH_BUTTON_PRESSED, DOUBLE_CLICK, ENABLE_MOUSE_INPUT, ENABLE_WINDOW_INPUT,
    FOCUS_EVENT, INPUT_RECORD, KEY_EVENT, MOUSE_EVENT, MOUSE_HWHEELED, MOUSE_MOVED, MOUSE_WHEELED,
    RIGHTMOST_BUTTON_PRESSED,
};
use windows::winapi::um::winuser::GetKeyState;
use windows::winapi::um::winuser::VK_LSHIFT;
use windows::winapi::um::winuser::VK_RSHIFT;
use windows::window::WindowsWindow;
use windows::Empty;

#[allow(dead_code)]
pub struct WindowsApplication {
    window: WindowsWindow,
    terminal: WindowsTerminal,
    mouse: WindowsMouse,
    event_queue: EventQueue,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

#[allow(dead_code)]
impl WindowsApplication {
    pub fn create() -> Result<WindowsApplication> {
        let application = WindowsApplication {
            window: WindowsWindow::new(),
            terminal: WindowsTerminal::create()?,
            mouse: WindowsMouse::new(),
            event_queue: EventQueue::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        };

        let success = unsafe {
            SetConsoleMode(
                application.terminal.input_handle,
                ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT,
            )
        };

        if success == -1 {
            return Err("Couldn't set the console mode.");
        }

        Ok(application)
    }
}

impl Application for WindowsApplication {
    #[inline]
    fn get_terminal(&self) -> &Terminal {
        &self.terminal
    }

    #[inline]
    fn get_window(&self) -> &Window {
        &self.window
    }

    #[inline]
    fn get_mouse(&self) -> &Mouse {
        &self.mouse
    }

    #[inline]
    fn get_mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    #[inline]
    fn get_keyboard_state(&self) -> &KeyboardState {
        &self.keyboard_state
    }

    #[inline]
    fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    #[inline]
    fn get_mut_event_queue(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }

    fn listen_events(&mut self) -> Result<()> {
        let mut input_records = [INPUT_RECORD::empty(); 128];
        let mut events_read: u32 = 0;

        let success =
            unsafe { GetNumberOfConsoleInputEvents(self.terminal.input_handle, &mut events_read) };

        if success == -1 {
            return Err("Couldn't determine the amount of unread events");
        }

        if events_read <= 0 {
            return Ok(());
        }

        let success = unsafe {
            ReadConsoleInputW(
                self.terminal.input_handle,
                input_records.as_mut_ptr(),
                128,
                &mut events_read,
            )
        };

        if success == -1 {
            return Err("Couldn't retrieve the console window events.");
        }

        for input_record in input_records.iter() {
            let event = match input_record.EventType {
                KEY_EVENT => {
                    let event = process_key_events(input_record);
                    self.event_queue.add_event(event);
                    event
                }
                MOUSE_EVENT => {
                    let event = process_mouse_events(input_record);
                    self.event_queue.add_event(event);
                    event
                }
                FOCUS_EVENT => {
                    let event = process_window_events(&self.window)?;
                    self.event_queue.add_event(event);
                    event
                }
                _ => continue,
            };

            match event {
                Event::Mouse(mouse) => self.mouse_state.update_from_event(mouse),
                Event::Keyboard(keyboard) => self.keyboard_state.update_from_event(keyboard),
                Event::Window(_) => (),
            }
        }

        Ok(())
    }
}

#[inline]
fn process_key_events(input_record: &INPUT_RECORD) -> Event {
    let keyboard_event = unsafe { input_record.Event.KeyEvent() };

    Event::Keyboard(KeyboardEvent {
        event_type: if keyboard_event.bKeyDown == -1 {
            KeyboardEventType::KeyUp
        } else {
            KeyboardEventType::KeyDown
        },
        key_pressed: keyboard_event.wVirtualKeyCode,
        character: get_char_from_u16(unsafe { *keyboard_event.uChar.UnicodeChar() }),
        left_control: keyboard_event.dwControlKeyState & LEFT_CTRL_PRESSED != 0,
        left_shift: unsafe { GetKeyState(VK_LSHIFT) as u16 } & 0x8000 != 0,
        left_alt: keyboard_event.dwControlKeyState & LEFT_ALT_PRESSED != 0,
        right_control: keyboard_event.dwControlKeyState & RIGHT_CTRL_PRESSED != 0,
        right_shift: unsafe { GetKeyState(VK_RSHIFT) as u16 } & 0x8000 != 0,
        right_alt: keyboard_event.dwControlKeyState & RIGHT_ALT_PRESSED != 0,
    })
}

#[inline]
fn process_mouse_events(input_record: &INPUT_RECORD) -> Event {
    let mouse_event = unsafe { input_record.Event.MouseEvent() };

    Event::Mouse(MouseEvent {
        event_type: match mouse_event.dwEventFlags {
            0 => MouseEventType::Click,
            MOUSE_MOVED => MouseEventType::MouseMove,
            MOUSE_WHEELED => MouseEventType::HorizontalWheel,
            MOUSE_HWHEELED => MouseEventType::VerticalWheel,
            DOUBLE_CLICK => MouseEventType::DoubleClick,
            _ => MouseEventType::MouseMove,
        },
        left_button: mouse_event.dwButtonState & FROM_LEFT_1ST_BUTTON_PRESSED != 0,
        middle_button: mouse_event.dwButtonState & FROM_LEFT_2ND_BUTTON_PRESSED != 0,
        right_button: mouse_event.dwButtonState & RIGHTMOST_BUTTON_PRESSED != 0,
        extra_button_1: mouse_event.dwButtonState & FROM_LEFT_3RD_BUTTON_PRESSED != 0,
        extra_button_2: mouse_event.dwButtonState & FROM_LEFT_4TH_BUTTON_PRESSED != 0,
        extra_button_3: false,
        extra_button_4: false,
        wheel_delta: get_wheel_delta(mouse_event.dwButtonState),
        position: Point2d::new(
            mouse_event.dwMousePosition.X as usize,
            mouse_event.dwMousePosition.Y as usize,
        ),
    })
}

#[inline]
fn process_window_events(window: &WindowsWindow) -> Result<Event> {
    Ok(Event::Window(WindowEvent {
        event_type: WindowEventType::WindowFocus,
        position: window.get_window_position()?,
        size: window.get_window_size()?,
    }))
}

#[inline]
fn get_wheel_delta(button_state: u32) -> i16 {
    (button_state >> 16) as i16
}

#[inline]
fn get_char_from_u16(unicode: u16) -> char {
    match from_u32(unicode as u32) {
        Some(character) => character,
        None => ' ',
    }
}
