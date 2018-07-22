use std::time::{Duration, Instant};

mod core;
mod windows;
use core::cell::Cell;
use core::cell_buffer::CellBuffer;
use core::color::Color;
use core::mouse::Mouse;
use core::terminal::Terminal;
use windows::mouse::WindowsMouse;
use windows::terminal::WindowsTerminal;

struct Player {
    ox: f32,
    oy: f32,
    nx: f32,
    ny: f32,
}

fn check_size(terminal: &WindowsTerminal, buffer: &mut CellBuffer) {
    let size = terminal.get_console_size();

    if size.0 != buffer.width || size.1 != buffer.height {
        buffer.resize(Cell::new(' ', Color::Black, Color::Green), size.0, size.1);
    }
}

fn draw_stats(terminal: &WindowsTerminal, buffer: &mut CellBuffer, fps: i32) {
    let text_background = Cell::new(' ', Color::White, Color::DarkGreen);
    let separator = Cell::new('=', Color::White, Color::DarkGreen);
    let square = CellBuffer::new(Cell::new('#', Color::Grey, Color::DarkGreen), 5, 5);

    let console_size = terminal.get_console_size();
    let window_size = terminal.get_window_client_size();
    let char_size = terminal.get_char_size();

    buffer.write_cell_buffer(&square, 10, 10);
    buffer.repeat_cell(text_background, 0, 0, console_size.0);
    buffer.repeat_cell(separator, 0, 1, console_size.0);
    buffer.write_string(
        &format!(
            "FPS: {}   Window({}, {})   Console({}, {})   Char({}, {})",
            fps,
            window_size.0,
            window_size.1,
            console_size.0,
            console_size.1,
            char_size.0,
            char_size.1
        ),
        0,
        0,
        Color::White,
        text_background.background,
    );
}

fn draw_player(terminal: &WindowsTerminal, buffer: &mut CellBuffer, player: &mut Player) {
    let console_size = terminal.get_console_size();
    let player_cell = Cell::new('@', Color::Green, Color::Blue);
    let background_cell = Cell::new(' ', Color::Black, Color::Green);

    player.nx = player.ox + 0.01;

    if player.nx as usize == console_size.0 {
        player.ny = player.oy + 1.0;
        player.nx = 0.0;
    }

    if player.ny as usize == console_size.1 {
        player.ny = 2.0;
    }

    buffer.set(player.ox as usize, player.oy as usize, background_cell);
    buffer.set(player.nx as usize, player.ny as usize, player_cell);

    player.ox = player.nx;
    player.oy = player.ny;
}

fn draw_mouse(terminal: &WindowsTerminal, buffer: &mut CellBuffer, mouse: &WindowsMouse) {
    let position = mouse.get_client_position();
    let char_size = terminal.get_char_size();
    let cursor = Cell::new('▓', Color::White, Color::Black);

    if char_size.0 == 0 || char_size.1 == 0 {
        return;
    }

    buffer.set(position.0 / char_size.0, position.1 / char_size.1, cursor);
}

fn main() {
    let mouse = WindowsMouse::new();
    mouse.show_cursor(false);
    mouse.show_cursor(true);
    mouse.show_cursor(false);
    mouse.show_cursor(false);

    let terminal = WindowsTerminal::new();
    terminal.set_window_position(0, 0);
    terminal.set_window_size(800, 600);
    terminal.clear();
    terminal.set_cursor(0, 0);
    terminal.set_cursor_visibility(false);

    let mut buffer = CellBuffer::new(Cell::new(' ', Color::Black, Color::Green), 0, 0);

    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    let mut player = Player {
        ox: 0.0,
        oy: 0.0,
        nx: 0.0,
        ny: 2.0,
    };

    loop {
        let now = Instant::now();

        frames += 1;

        // checks the size and resize the buffer if required.
        check_size(&terminal, &mut buffer);

        // checks the app stats and draw them in the stat bar.
        draw_stats(&terminal, &mut buffer, fps);

        // draws the moving player.
        draw_player(&terminal, &mut buffer, &mut player);

        // draws the mouse cursor.
        draw_mouse(&terminal, &mut buffer, &mouse);

        // blits the buffer onto the terminal console.
        terminal.draw(&buffer);

        // checks the frames.
        duration += now.elapsed();

        if duration.as_secs() > 1 {
            duration = Duration::from_micros(0);
            fps = frames;
            frames = 0;
        }
    }

    terminal.dispose();
}
