extern crate winapi;
use super::super::core::mouse::Mouse;
use super::Empty;
use windows::winapi::shared::windef::{HWND,POINT,HCURSOR};
use windows::winapi::um::wincon::GetConsoleWindow;
use windows::winapi::um::winuser::{GetCursorPos, ScreenToClient, SetCursorPos, SetCursor, LoadCursorW, IDC_ARROW};
use std::ptr::null_mut;

#[derive(Debug)]
pub struct WindowsMouse {
    window_handle: HWND,
}

impl Mouse for WindowsMouse {
    fn new() -> WindowsMouse {
        WindowsMouse {
            window_handle: unsafe { GetConsoleWindow() },
        }
    }

    fn get_absolute_position(&self) -> (usize, usize) {
        let mut point = POINT::empty();
        let success = unsafe { GetCursorPos(&mut point) };

        if success == 0 {
            panic!("Problems trying to obtain the cursor position.");
        }

        (point.x as usize, point.y as usize)
    }

    fn get_client_position(&self) -> (usize, usize) {
        let position = self.get_absolute_position();
        let mut point = POINT {
            x: position.0 as i32,
            y: position.1 as i32,
        };

        let success = unsafe { ScreenToClient(self.window_handle, &mut point) };

        if success == 0 {
            panic!("Problems trying to obtain the client cursor position.");
        }

        (point.x as usize, point.y as usize)
    }

    fn set_position(&self, x: usize, y: usize) {
        let success = unsafe { SetCursorPos(x as i32, y as i32) };

        if success == 0 {
            panic!("Problems trying to set the cursor position.");
        }
    }

    fn show_cursor(&self, visible: bool) {
        if visible {
            unsafe { SetCursor(LoadCursorW(null_mut(), IDC_ARROW)) }
        }
        else {
            unsafe { SetCursor(null_mut()) }
        };
    }
}
