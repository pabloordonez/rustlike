extern crate winapi;
use windows::winapi::ctypes::c_void;
use windows::winapi::shared::windef::HWND;
use windows::winapi::um::fileapi::CreateFileW;
use windows::winapi::um::handleapi::CloseHandle;
use windows::winapi::um::processenv::GetStdHandle;
use windows::winapi::um::winbase::STD_OUTPUT_HANDLE;
use windows::winapi::um::wincon::{
    GetConsoleCursorInfo, GetConsoleScreenBufferInfo, GetConsoleWindow, SetConsoleCursorInfo,
    SetConsoleCursorPosition, WriteConsoleOutputW, CHAR_INFO, CONSOLE_CURSOR_INFO,
    CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
};
use windows::winapi::um::winnt::HANDLE;

use std::mem::zeroed;
use std::ptr::null_mut;

use super::super::core::cell::Cell;
use super::super::core::cell_buffer::CellBuffer;
use super::super::core::point_2d::Point2d;
use super::super::core::size_2d::Size2d;
use super::super::core::terminal::Terminal;
use super::super::core::window::Window;
use super::{get_background_color, get_foreground_color, get_wstring, Empty};

#[derive(Debug)]
pub struct WindowsTerminal {
    pub console_handle: *mut c_void,
    pub output_handle: HANDLE,
    pub window_handle: HWND,
}

impl WindowsTerminal {
    pub fn new() -> WindowsTerminal {
        WindowsTerminal {
            console_handle: unsafe {
                CreateFileW(
                    get_wstring("CONOUT$").as_ptr(),
                    0x40000000,
                    2,
                    null_mut(),
                    3,
                    0,
                    null_mut(),
                )
            },
            output_handle: unsafe { GetStdHandle(STD_OUTPUT_HANDLE) },
            window_handle: unsafe { GetConsoleWindow() },
        }
    }
}

#[allow(dead_code)]
impl Terminal for WindowsTerminal {
    fn dispose(&self) {
        unsafe { CloseHandle(self.console_handle) };
    }

    fn set_cursor_visibility(&self, visible: bool) {
        let mut console_cursor_info = CONSOLE_CURSOR_INFO::empty();
        let success = unsafe { GetConsoleCursorInfo(self.output_handle, &mut console_cursor_info) };

        if success == 0 {
            panic!("Problems trying to obtain the console cursor info.");
        }

        console_cursor_info.bVisible = if visible { 1 } else { 0 };

        let success = unsafe { SetConsoleCursorInfo(self.output_handle, &mut console_cursor_info) };

        if success == 0 {
            panic!("Problems trying to set the console cursor info.");
        }
    }

    fn set_cursor(&self, position: Point2d) {
        let success: i32 = unsafe {
            SetConsoleCursorPosition(
                self.output_handle,
                COORD {
                    X: position.x as i16,
                    Y: position.y as i16,
                },
            )
        };

        if success == 0 {
            panic!("Couldn't set the console cursor position.");
        }
    }

    fn get_console_size(&self) -> Size2d {
        let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::empty();
        let success = unsafe {
            GetConsoleScreenBufferInfo(self.output_handle, &mut console_screen_buffer_info)
        };

        if success == 0 {
            panic!("Problems trying to obtain the screen buffer info.");
        }

        let window = console_screen_buffer_info.srWindow;
        Size2d::new(
            (window.Right - window.Left + 1) as usize,
            (window.Bottom - window.Top + 1) as usize,
        )
    }

    fn get_char_size(&self, window: &Window) -> Size2d {
        let console_size = self.get_console_size();
        let client_size = window.get_window_client_size();

        if console_size.is_empty() {
            return Size2d::empty();
        }

        Size2d::new(
            client_size.width / console_size.width,
            client_size.height / console_size.height,
        )
    }

    fn clear(&self) {
        let size = self.get_console_size();
        let width = size.width as i16;
        let height = size.height as i16;
        let mut char_info: CHAR_INFO;

        unsafe {
            char_info = zeroed();
            char_info.Attributes = 0;
            *char_info.Char.UnicodeChar_mut() = ' ' as u16;
        }

        let char_info_array = vec![char_info; (width * height) as usize];

        let mut rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: width as i16,
            Bottom: height as i16,
        };

        let success = unsafe {
            WriteConsoleOutputW(
                self.console_handle,
                char_info_array.as_ptr(),
                COORD {
                    X: width as i16,
                    Y: height as i16,
                },
                COORD::empty(),
                &mut rect as *mut SMALL_RECT,
            )
        };

        if success == 0 {
            panic!("Couldn't clear console output.");
        }
    }

    fn write(&self, cell_buffer: &CellBuffer) {
        let char_info_array = cell_buffer
            .iter()
            .map(|cell: &Cell| unsafe {
                let mut char_info: CHAR_INFO = zeroed();
                char_info.Attributes =
                    get_foreground_color(cell.foreground) | get_background_color(cell.background);
                *char_info.Char.UnicodeChar_mut() = cell.character as u16;
                char_info
            })
            .collect::<Vec<CHAR_INFO>>();

        let mut rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: cell_buffer.size.width as i16,
            Bottom: cell_buffer.size.height as i16,
        };

        let success = unsafe {
            WriteConsoleOutputW(
                self.console_handle,
                char_info_array.as_ptr(),
                COORD {
                    X: cell_buffer.size.width as i16,
                    Y: cell_buffer.size.height as i16,
                },
                COORD::empty(),
                &mut rect as *mut SMALL_RECT,
            )
        };

        if success == 0 {
            panic!("Couldn't write to the console output.");
        }
    }
}
