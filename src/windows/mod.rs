extern crate winapi;

use windows::winapi::shared::windef::{POINT, RECT};
use windows::winapi::um::wincon::{
    CHAR_INFO, CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
};

use std::mem::zeroed;

pub mod cell;

pub mod cell_buffer;

pub mod window;

pub mod terminal;

pub mod mouse;

pub trait Empty {
    fn empty() -> Self;
}

impl Empty for CHAR_INFO {
    fn empty() -> CHAR_INFO {
        unsafe { zeroed::<CHAR_INFO>() }
    }
}

impl Empty for POINT {
    fn empty() -> POINT {
        POINT { x: 0, y: 0 }
    }
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

impl Empty for RECT {
    fn empty() -> RECT {
        RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
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
            bVisible: 0,
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
