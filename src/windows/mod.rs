extern crate winapi;

use windows::winapi::um::wincon::{
    CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_CURSOR_INFO, COORD, SMALL_RECT,
};

use super::core::color::Color;

pub mod terminal;

pub trait Empty {
    fn empty() -> Self;
}

impl Empty for COORD {
    fn empty() -> COORD {
        COORD { X: 0, Y: 0 }
    }
}

impl Empty for SMALL_RECT {
    fn empty() -> SMALL_RECT {
        SMALL_RECT {
            Top: 0,
            Right: 0,
            Bottom: 0,
            Left: 0,
        }
    }
}

impl Empty for CONSOLE_SCREEN_BUFFER_INFO {
    fn empty() -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD::empty(),
            dwCursorPosition: COORD::empty(),
            wAttributes: 0,
            srWindow: SMALL_RECT::empty(),
            dwMaximumWindowSize: COORD::empty(),
        }
    }
}

impl Empty for CONSOLE_CURSOR_INFO {
    fn empty() -> CONSOLE_CURSOR_INFO {
        CONSOLE_CURSOR_INFO {
            dwSize: 0,
            bVisible: 0
        }
    }
}

#[inline]
pub fn get_wstring(msg: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(msg).encode_wide().chain(once(0)).collect()
}

#[inline]
pub fn get_foreground_color(color: Color) -> u16
{
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
        Color::Red=> 12,
        Color::Magenta => 13,
        Color::Yellow => 14,
        Color::White => 15
    }
}

#[inline]
pub fn get_background_color(color: Color) -> u16
{
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
        Color::Red=> 192,
        Color::Magenta => 208,
        Color::Yellow => 224,
        Color::White => 240
    }
}