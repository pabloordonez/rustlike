use std::time::{Duration, Instant};

mod core;
mod windows;
use core::cell::Cell;
use core::cellbuffer::CellBuffer;
use core::color::Color;
use core::terminal::Terminal;
use windows::terminal::WindowsTerminal;

fn main() {
    let terminal = WindowsTerminal::new();
    terminal.set_window_position(0, 0);
    terminal.set_window_size(800, 600);
    terminal.clear();
    terminal.set_cursor(0, 0);
    terminal.set_cursor_visibility(false);

    let size = terminal.get_console_size();

    let player = Cell::new('@', Color::Green, Color::Blue);
    let background = Cell::new(' ', Color::Black, Color::Green);
    let text_background = Cell::new(' ', Color::White, Color::DarkGreen);
    let separator = Cell::new('=', Color::White, Color::DarkGreen);

    let mut buffer = CellBuffer::new(background, size.0, size.1);
    let square = CellBuffer::new(Cell::new('#', Color::Grey, Color::DarkGreen), 5, 5);

    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    let mut ox: f32 = 0.0;
    let mut oy: f32 = 0.0;
    let mut nx: f32;
    let mut ny: f32 = 2.0;

    loop {
        let now = Instant::now();
        frames += 1;
        nx = ox + 0.01;

        if nx as usize == size.0 {
            ny = oy + 1.0;
            nx = 0.0;
        }

        if ny as usize == size.1 {
            ny = 2.0;
        }

        buffer.set(ox as usize, oy as usize, background);
        buffer.set(nx as usize, ny as usize, player);

        buffer.write_cell_buffer(&square, 10, 10);

        buffer.repeat_cell(text_background, 0, 0, size.0);
        buffer.write_string(
            &format!("FPS: {}       X: {} Y: {}", fps, nx as usize, ny as usize),
            0,
            0,
            Color::White,
            text_background.background,
        );

        buffer.repeat_cell(separator, 0, 1, size.0);

        terminal.draw(&buffer);

        ox = nx;
        oy = ny;

        duration += now.elapsed();

        if duration.as_secs() > 1 {
            duration = Duration::from_micros(0);
            fps = frames;
            frames = 0;
        }
    }

    terminal.dispose();
}
