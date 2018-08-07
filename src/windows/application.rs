use core::application::Application;
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
use windows::winapi::um::consoleapi::{GetNumberOfConsoleInputEvents, ReadConsoleInputW, SetConsoleMode};
use windows::winapi::um::wincon::{FOCUS_EVENT, INPUT_RECORD, KEY_EVENT, MOUSE_EVENT, ENABLE_WINDOW_INPUT, ENABLE_MOUSE_INPUT};
use windows::window::WindowsWindow;
use windows::Empty;

#[allow(dead_code)]
pub struct WindowsApplication {
    window: WindowsWindow,
    terminal: WindowsTerminal,
    mouse: WindowsMouse,
    event_queue: EventQueue,
}

#[allow(dead_code)]
impl WindowsApplication {
    pub fn create() -> Result<WindowsApplication> {

        let application = WindowsApplication {
            window: WindowsWindow::new(),
            terminal: WindowsTerminal::create()?,
            mouse: WindowsMouse::new(),
            event_queue: EventQueue::new(),
        };

        let success = unsafe {
            SetConsoleMode(application.terminal.input_handle, ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT)
        };

        if success == -1 {
            return Err("Couldn't set the console mode.");
        }

        Ok(application)
    }
}

impl Application for WindowsApplication {
    fn get_terminal(&self) -> &Terminal {
        &self.terminal
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn get_mouse(&self) -> &Mouse {
        &self.mouse
    }

    fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

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
            match input_record.EventType {
                KEY_EVENT => self.event_queue.add_event(process_key_events(input_record)),
                MOUSE_EVENT => self.event_queue.add_event(process_mouse_events(input_record)),
                FOCUS_EVENT => self.event_queue.add_event(process_window_events(&self.window)?),
                _ => continue,
            };
        }

        Ok(())
    }
}

fn process_key_events(input_record: &INPUT_RECORD) -> Event {
    let keyboard_event = unsafe { input_record.Event.KeyEvent() };

    Event::Keyboard(KeyboardEvent {
        event_type: KeyboardEventType::KeyDown,
        key_pressed: keyboard_event.wVirtualKeyCode,
        character: get_char_from_u16(unsafe { *keyboard_event.uChar.UnicodeChar() }),
        control: false,
        shift: false,
        alt: false,
    })
}

fn process_mouse_events(input_record: &INPUT_RECORD) -> Event {
    let mouse_event = unsafe { input_record.Event.MouseEvent() };

    Event::Mouse(MouseEvent {
        event_type: MouseEventType::Click,
        button_pressed: mouse_event.dwButtonState as u8,
        position: Point2d::new(
            mouse_event.dwMousePosition.X as usize,
            mouse_event.dwMousePosition.X as usize,
        ),
    })
}

fn process_window_events(window: &WindowsWindow) -> Result<Event> {
    Ok(Event::Window(WindowEvent {
        event_type: WindowEventType::WindowFocus,
        position: window.get_window_position()?,
        size: window.get_window_size()?,
    }))
}

fn get_char_from_u16(unicode: u16) -> char {
    match from_u32(unicode as u32) {
        Some(character) => character,
        None => ' ',
    }
}
