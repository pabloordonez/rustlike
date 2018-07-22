use super::core::CellBuffer;
use super::core::Color as CoreColor;

extern crate crossterm;
use crossterm::crossterm::cursor;
use crossterm::crossterm::cursor::TerminalCursor;
use crossterm::crossterm::style::Color;
use crossterm::crossterm::terminal::terminal::Terminal as CrossTerminal;
use crossterm::crossterm::terminal::{terminal, ClearType};
use crossterm::crossterm::Context;

use std::io::{stdout, Write};

pub struct Terminal {
    terminal: Box<CrossTerminal>,
    cursor: Box<TerminalCursor>,
}

fn get_color(color: CoreColor) -> Color {
    match color {
        CoreColor::Black => Color::Black,
        CoreColor::Red => Color::Red,
        CoreColor::DarkRed => Color::DarkRed,
        CoreColor::Green => Color::Green,
        CoreColor::DarkGreen => Color::DarkGreen,
        CoreColor::Yellow => Color::Yellow,
        CoreColor::DarkYellow => Color::DarkYellow,
        CoreColor::Blue => Color::Blue,
        CoreColor::DarkBlue => Color::DarkBlue,
        CoreColor::Magenta => Color::Magenta,
        CoreColor::DarkMagenta => Color::DarkMagenta,
        CoreColor::Cyan => Color::Cyan,
        CoreColor::DarkCyan => Color::DarkCyan,
        CoreColor::Grey => Color::Grey,
        CoreColor::White => Color::White,
    }
}

impl Terminal {
    pub fn new() -> Terminal {
        let context = &Context::new();
        let terminal = Terminal {
            terminal: terminal::terminal(context),
            cursor: cursor::cursor(context),
        };

        terminal.cursor.hide();
        return terminal;
    }

    pub fn clear_screen(&self) {
        self.terminal.clear(ClearType::All);
    }

    pub fn get_size(&self) -> (usize, usize) {
        let size = self.terminal.terminal_size();
        (size.0 as usize, size.1 as usize)
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor.goto(x, y);
    }

    pub fn print_cell_buffer(&self, cell_buffer: &CellBuffer) {
        let stdout = stdout();
        let mut lock = stdout.lock();

        for row in cell_buffer.cells.iter() {
            for cell in row.iter() {
                let o = self.terminal
                    .paint(cell.character)
                    .with(get_color(cell.foreground))
                    .on(get_color(cell.background));

                lock.write(o.to_string().as_bytes()).unwrap();
            }
        }
    }
}
