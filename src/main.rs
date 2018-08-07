use std::time::{Duration, Instant};

mod core;
mod windows;
use core::application::Application;
use core::cell::Cell;
use core::cell_buffer::CellBuffer;
use core::color::Color;
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use core::Result;
use windows::application::WindowsApplication;

struct Player {
    ox: f32,
    oy: f32,
    nx: f32,
    ny: f32,
}

fn check_size(application: &Application, buffer: &mut CellBuffer) {
    let size = application.get_terminal().get_console_size().unwrap();

    if size.width != buffer.size.width || size.height != buffer.size.height {
        buffer.resize(Cell::new(' ', Color::Black, Color::Green), size);
    }
}

fn draw_stats(application: &Application, buffer: &mut CellBuffer, fps: i32) {
    let text_background = Cell::new(' ', Color::White, Color::DarkGreen);
    let separator = Cell::new('=', Color::White, Color::DarkGreen);
    let square = CellBuffer::new(
        Cell::new('#', Color::Grey, Color::DarkGreen),
        Size2d::new(5, 5),
    );

    let window = application.get_window();
    let terminal = application.get_terminal();

    let console_size = terminal.get_console_size().unwrap();
    let window_size = window.get_window_client_size().unwrap();
    let char_size = terminal.get_char_size(window).unwrap();

    buffer.write_cell_buffer(&square, Point2d::new(10, 10));
    buffer.repeat_cell(text_background, Point2d::new(0, 0), console_size.width);
    buffer.repeat_cell(separator, Point2d::new(0, 1), console_size.width);
    buffer.write_string(
        &format!(
            "FPS: {}   Window({}, {})   Console({}, {})   Char({}, {})",
            fps,
            window_size.width,
            window_size.height,
            console_size.width,
            console_size.height,
            char_size.width,
            char_size.height
        ),
        Point2d::empty(),
        Color::White,
        text_background.background,
    );
}

fn draw_player(application: &Application, buffer: &mut CellBuffer, player: &mut Player) {
    let console_size = application.get_terminal().get_console_size().unwrap();
    let player_cell = Cell::new('@', Color::Green, Color::Blue);
    let background_cell = Cell::new(' ', Color::Black, Color::Green);

    player.nx = player.ox + 0.01;

    if player.nx as usize == console_size.width {
        player.ny = player.oy + 1.0;
        player.nx = 0.0;
    }

    if player.ny as usize == console_size.height {
        player.ny = 3.0;
    }

    buffer.set(
        Point2d::new(player.ox as usize, player.oy as usize),
        background_cell,
    );
    buffer.set(
        Point2d::new(player.nx as usize, player.ny as usize),
        player_cell,
    );

    player.ox = player.nx;
    player.oy = player.ny;
}

fn draw_mouse(application: &Application, buffer: &mut CellBuffer) {
    let position = application.get_mouse().get_client_position().unwrap();
    let char_size = application
        .get_terminal()
        .get_char_size(application.get_window())
        .unwrap();
    let cursor = Cell::new('▓', Color::White, Color::Black);

    if char_size.is_empty() {
        return;
    }

    buffer.set(
        Point2d::new(position.x / char_size.width, position.y / char_size.height),
        cursor,
    );
}

fn main() -> Result<()> {
    let mut application = WindowsApplication::create()?;

    {
        let window = application.get_window();
        window.set_window_position(Point2d::empty())?;
        window.set_window_size(Size2d::new(800, 600))?;

        let mouse = application.get_mouse();
        mouse.show_cursor(false)?;

        let terminal = application.get_terminal();
        terminal.clear()?;
        terminal.set_cursor(Point2d::empty())?;
        terminal.set_cursor_visibility(false)?;
    }

    let mut buffer = CellBuffer::new(Cell::new(' ', Color::Black, Color::Green), Size2d::empty());

    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    let mut player = Player {
        ox: 0.0,
        oy: 0.0,
        nx: 0.0,
        ny: 3.0,
    };

    loop {
        let now = Instant::now();

        frames += 1;

        application.listen_events()?;

        let mut i = 0;

        while let Some(event) = application.get_mut_event_queue().get_event() {
            buffer.write_string(&format!("{:?}", event), Point2d::new(0, 2 + i), Color::White, Color::Black);
            i = i + 1;
        }

        // checks the size and resize the buffer if required.
        check_size(&application, &mut buffer);

        // checks the app stats and draw them in the stat bar.
        draw_stats(&application, &mut buffer, fps);

        // draws the moving player.
        draw_player(&application, &mut buffer, &mut player);

        // draws the mouse cursor.
        draw_mouse(&application, &mut buffer);

        // blits the buffer onto the terminal console.
        application.get_terminal().write(&buffer)?;

        // checks the frames.
        duration += now.elapsed();

        if duration.as_secs() > 1 {
            duration = Duration::from_micros(0);
            fps = frames;
            frames = 0;
        }
    }

    //application.get_terminal().dispose()?;
}
