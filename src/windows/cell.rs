extern crate winapi;

use core::cell::Cell;
use core::color::Color;
use std::char::from_u32;
use core::windows::Empty;
use windows::winapi::um::wincon::CHAR_INFO;

pub trait NewCharInfo{
    fn new(character: char, fg: Color, bg: Color) -> CHAR_INFO;
}

impl NewCharInfo for CHAR_INFO {
    fn new(character: char, fg: Color, bg: Color) -> CHAR_INFO {
        let char_info = CHAR_INFO::empty();

        char_info.set_char(character);
        char_info.set_colors(fg, bg);

        char_info
    }
}

impl Cell for CHAR_INFO {
    #[inline]
    fn set_char(&self, character: char) {
        *self.Char.UnicodeChar_mut() = character as u16;
    }

    #[inline]
    fn get_char(&self) -> char {
        match from_u32(*self.Char.UnicodeChar_mut() as u32) {
            Some(character) => character,
            None => ' ',
        }
    }

    #[inline]
    fn set_colors(&self, foreground: Color, background: Color) {
        self.Attributes = (get_u16_from_color(background) << 4) | get_u16_from_color(foreground);
    }

    #[inline]
    fn set_bg_color(&self, color: Color) {
        self.Attributes = (self.Attributes & 0x0F) | (get_u16_from_color(color) << 4);
    }

    #[inline]
    fn set_fg_color(&self, color: Color) {
        self.Attributes = (self.Attributes & 0xF0) | get_u16_from_color(color);
    }

    #[inline]
    fn get_bg_color(&self) -> Color {
        get_color_from_u16(self.Attributes >> 4)
    }

    #[inline]
    fn get_fg_color(&self) -> Color {
        get_color_from_u16(self.Attributes)
    }
}

#[inline]
fn get_color_from_u16(color: u16) -> Color {
    match color & 0x0F {
        0 => Color::Black,
        1 => Color::DarkBlue,
        2 => Color::DarkGreen,
        3 => Color::DarkCyan,
        4 => Color::DarkRed,
        5 => Color::DarkMagenta,
        6 => Color::DarkYellow,
        7 => Color::Grey,
        8 => Color::DarkGrey,
        9 => Color::Blue,
        10 => Color::Green,
        11 => Color::Cyan,
        12 => Color::Red,
        13 => Color::Magenta,
        14 => Color::Yellow,
        15 => Color::White,
        _ => Color::Black,
    }
}

#[inline]
fn get_u16_from_color(color: Color) -> u16 {
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
