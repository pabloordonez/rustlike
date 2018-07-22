extern crate winapi;
use windows::winapi::ctypes::c_void;
use windows::winapi::shared::windef::{HWND, RECT};
use windows::winapi::um::fileapi::CreateFileW;
use windows::winapi::um::handleapi::CloseHandle;
use windows::winapi::um::processenv::GetStdHandle;
use windows::winapi::um::winbase::STD_OUTPUT_HANDLE;
use windows::winapi::um::wincon::{
    CHAR_INFO_Char, GetConsoleCursorInfo, GetConsoleScreenBufferInfo, GetConsoleWindow,
    SetConsoleCursorInfo, SetConsoleCursorPosition, WriteConsoleOutputW, CHAR_INFO,
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
};
use windows::winapi::um::winnt::HANDLE;
use windows::winapi::um::winuser::{GetClientRect, GetWindowRect, SetWindowPos};

use std::mem::transmute;
use std::ptr::null_mut;

use super::super::core::cell::Cell;
use super::super::core::cell_buffer::CellBuffer;
use super::super::core::terminal::Terminal;
use super::{get_background_color, get_foreground_color, get_wstring, Empty};

#[derive(Debug)]
pub struct WindowsTerminal {
    pub console_handle: *mut c_void,
    pub output_handle: HANDLE,
    pub window_handle: HWND,
}

#[allow(dead_code)]
impl Terminal for WindowsTerminal {
    fn new() -> WindowsTerminal {
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

    fn set_cursor(&self, x: u16, y: u16) {
        let success: i32 = unsafe {
            SetConsoleCursorPosition(
                self.output_handle,
                COORD {
                    X: x as i16,
                    Y: y as i16,
                },
            )
        };

        if success == 0 {
            panic!("Couldn't set the console cursor position.");
        }
    }

    fn get_console_size(&self) -> (usize, usize) {
        let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::empty();
        let success = unsafe {
            GetConsoleScreenBufferInfo(self.output_handle, &mut console_screen_buffer_info)
        };

        if success == 0 {
            panic!("Problems trying to obtain the screen buffer info.");
        }

        let window = console_screen_buffer_info.srWindow;

        (
            (window.Right - window.Left + 1) as usize,
            (window.Bottom - window.Top + 1) as usize,
        )
    }

    fn get_window_size(&self) -> (usize, usize) {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            panic!("Problems trying to obtain the window rect.");
        }

        (
            (rect.right - rect.left) as usize,
            (rect.bottom - rect.top) as usize,
        )
    }

    fn get_window_client_size(&self) -> (usize, usize) {
        let mut rect = RECT::empty();
        let success = unsafe { GetClientRect(self.window_handle, &mut rect) };

        if success == 0 {
            panic!("Problems trying to obtain the client rect.");
        }

        (
            (rect.right - rect.left) as usize,
            (rect.bottom - rect.top) as usize,
        )
    }

    fn get_char_size(&self) -> (usize, usize) {
        let console_size = self.get_console_size();
        let client_size = self.get_window_client_size();

        if console_size.0 == 0 || console_size.1 == 0 {
            return (0, 0);
        }

        (
            client_size.0 / console_size.0,
            client_size.1 / console_size.1,
        )
    }

    fn set_window_size(&self, width: usize, height: usize) {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            panic!("Problems trying to obtain the window rect.");
        }

        let success = unsafe {
            SetWindowPos(
                self.window_handle,
                0 as HWND,
                rect.top,
                rect.left,
                width as i32,
                height as i32,
                0x0020 | 0x0040,
            )
        };

        if success == 0 {
            panic!("Problem trying to set the windows size.");
        }
    }

    fn get_window_position(&self) -> (usize, usize) {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            panic!("Problems trying to obtain the window rect.");
        }

        (rect.left as usize, rect.top as usize)
    }

    fn set_window_position(&self, x: usize, y: usize) {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            panic!("Problems trying to obtain the window rect.");
        }

        let success = unsafe {
            SetWindowPos(
                self.window_handle,
                0 as HWND,
                x as i32,
                y as i32,
                rect.right - rect.left,
                rect.bottom - rect.top,
                0x0020 | 0x0040,
            )
        };

        if success == 0 {
            panic!("Problem trying to set the windows position.");
        }
    }

    fn clear(&self) {
        let size = self.get_console_size();
        let width = size.0;
        let height = size.1;

        let char_info_array = vec![
            CHAR_INFO {
                Attributes: 0,
                Char: unsafe { transmute::<u16, CHAR_INFO_Char>(' ' as u16) },
            };
            width * height
        ];

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

    fn draw(&self, cell_buffer: &CellBuffer) {
        let char_info_array = cell_buffer
            .cells
            .iter()
            .map(|cell: &Cell| CHAR_INFO {
                Attributes: get_foreground_color(cell.foreground)
                    | get_background_color(cell.background),
                Char: unsafe { transmute::<u16, CHAR_INFO_Char>(cell.character as u16) },
            })
            .collect::<Vec<CHAR_INFO>>();

        let mut rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: cell_buffer.width as i16,
            Bottom: cell_buffer.height as i16,
        };

        let success = unsafe {
            WriteConsoleOutputW(
                self.console_handle,
                char_info_array.as_ptr(),
                COORD {
                    X: cell_buffer.width as i16,
                    Y: cell_buffer.height as i16,
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
