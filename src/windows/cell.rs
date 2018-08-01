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

impl Cell for CHAR_INFO {
    fn set_char(&self, character: char) {
        *self.Char.UnicodeChar_mut() = character as u16;
    }

    fn set_colors(foreground: Color, background: Color) {
        char_info.Attributes =get_background_color(background) |  get_foreground_color(foreground);
    }

    fn set_bg_color(&self, color: Color) {
        char_info.Attributes = (char_info.Attributes & 0x00FF) | get_background_color(color);
    }

    fn set_fg_color(&self, color: Color) {
        har_info.Attributes = (char_info.Attributes & 0xFF00) | get_foreground_color(color);
    }

    fn get_bg_color(&self) -> Color {

    }

    fn get_fg_color(&self) -> Color {}
}

#[inline]
fn get_foreground_color(color: Color) -> u16 {
    match color {
        Color::Black => 0,
        Color::DarkBlue => 1,
        Color::DarkGreen => 2,
        Color::DarkCyan => 3,
        Color::DarkRed => 4,
        Color::DarkMagenta => 5,
        Color::DarkYellow => 6,
        Color::Grey => 7,
        Color::DarkGrey => 8,
        Color::Blue => 9,
        Color::Green => 10,
        Color::Cyan => 11,
        Color::Red => 12,
        Color::Magenta => 13,
        Color::Yellow => 14,
        Color::White => 15,
    }
}

#[inline]
fn get_background_color(color: Color) -> u16 {
    match color {
        Color::Black => 0,
        Color::DarkBlue => 16,
        Color::DarkGreen => 32,
        Color::DarkCyan => 48,
        Color::DarkRed => 64,
        Color::DarkMagenta => 80,
        Color::DarkYellow => 96,
        Color::Grey => 112,
        Color::DarkGrey => 128,
        Color::Blue => 144,
        Color::Green => 160,
        Color::Cyan => 176,
        Color::Red => 192,
        Color::Magenta => 208,
        Color::Yellow => 224,
        Color::White => 240,
    }
}
