extern crate winapi;
use core::drawing::point_2d::Point2d;
use core::mouse::Mouse;
use core::Result;
use std::ptr::null_mut;
use windows::winapi::shared::windef::{HWND, POINT};
use windows::winapi::um::wincon::GetConsoleWindow;
use windows::winapi::um::winuser::{
    GetCursorPos, LoadCursorW, ScreenToClient, SetCursor, SetCursorPos, IDC_ARROW,
};
use windows::Empty;

#[derive(Debug)]
pub struct WindowsMouse {
    window_handle: HWND,
}

impl WindowsMouse {
    pub fn new() -> WindowsMouse {
        WindowsMouse {
            window_handle: unsafe { GetConsoleWindow() },
        }
    }
}

impl Mouse for WindowsMouse {
    fn get_absolute_position(&self) -> Result<Point2d> {
        let mut point = POINT::empty();
        let success = unsafe { GetCursorPos(&mut point) };

        if success == 0 {
            return Err("Problems trying to obtain the cursor position.");
        }

        Ok(Point2d::new(point.x as usize, point.y as usize))
    }

    fn get_client_position(&self) -> Result<Point2d> {
        let position = self.get_absolute_position()?;
        let mut point = POINT {
            x: position.x as i32,
            y: position.y as i32,
        };

        let success = unsafe { ScreenToClient(self.window_handle, &mut point) };

        if success == 0 {
            return Err("Problems trying to obtain the client cursor position.");
        }

        Ok(Point2d::new(point.x as usize, point.y as usize))
    }

    fn set_position(&self, position: Point2d) -> Result<()> {
        let success = unsafe { SetCursorPos(position.x as i32, position.y as i32) };

        if success == 0 {
            return Err("Problems trying to set the cursor position.");
        }

        Ok(())
    }

    fn show_cursor(&self, visible: bool) -> Result<()> {
        if visible {
            unsafe { SetCursor(LoadCursorW(null_mut(), IDC_ARROW)) }
        } else {
            unsafe { SetCursor(null_mut()) }
        };

        Ok(())
    }
}
