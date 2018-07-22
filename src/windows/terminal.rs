extern crate winapi;
use windows::winapi::ctypes::c_void;
use windows::winapi::um::fileapi::CreateFileW;
use windows::winapi::um::handleapi::CloseHandle;
use windows::winapi::um::processenv::GetStdHandle;
use windows::winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use windows::winapi::um::wincon::{
    CHAR_INFO_Char, GetConsoleCursorInfo, GetConsoleScreenBufferInfo, SetConsoleCursorInfo,
    SetConsoleCursorPosition, WriteConsoleOutputW, CHAR_INFO, CONSOLE_CURSOR_INFO,
    CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
};
use windows::winapi::um::winnt::HANDLE;

use std::mem::transmute;
use std::ptr::null_mut;

use super::super::core::cell::Cell;
use super::super::core::cellbuffer::CellBuffer;
use super::super::core::terminal::Terminal;
use super::{get_background_color, get_foreground_color, get_wstring, Empty};

#[derive(Debug)]
pub struct WindowsTerminal {
    pub console_handle: *mut c_void,
    pub output_handle: HANDLE,
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

        if visible {
            console_cursor_info.bVisible = 1;
        } else {
            console_cursor_info.bVisible = 0;
        }

        let success = unsafe { SetConsoleCursorInfo(self.output_handle, &mut console_cursor_info) };

        if success == 0 {
            panic!("Problems trying to set the console cursor info.");
        }
    }

    fn set_cursor(&self, x: u16, y: u16) {
        let success: i32 =
            unsafe { SetConsoleCursorPosition(self.output_handle, COORD { X: x as i16, Y: y as i16 }) };

        if success == 0 {
            panic!("Couldn't set the console cursor position.");
        }
    }

    fn get_size(&self) -> (usize, usize) {
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

    fn set_size(&self, width: usize, height: usize) {}

    fn clear(&self) {
        let size = self.get_size();
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
