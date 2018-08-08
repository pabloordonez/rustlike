use core::mouse::is_right_button_pressed;
use core::mouse::is_left_button_pressed;
use std::time::{Duration, Instant};

mod core;
mod windows;
use core::application::Application;
use core::cell::Cell;
use core::cell_buffer::CellBuffer;
use core::color::Color;
use core::events::event::{Event, MouseEventType};
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use core::Result;
use windows::application::WindowsApplication;


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

    let mut buffer = CellBuffer::new(Cell::new_default(' '), Size2d::empty());
    let mut fps = 0;
    let mut frames = 0;
    let mut duration = Duration::from_micros(0);

    loop {
        let now = Instant::now();
        frames += 1;

        // process native events.
        application.listen_events()?;

        // while there is events in the event queue, process them.
        while let Some(event) = application.get_mut_event_queue().get_event() {
            match event {
                Event::Mouse(mouse) => {
                    if mouse.event_type == MouseEventType::MouseMove {
                        if is_left_button_pressed(mouse.buttons) {
                            buffer.set(mouse.position, Cell::new('░', Color::White, Color::Black));
                        }

                        if is_right_button_pressed(mouse.buttons) {
                            buffer.set(mouse.position, Cell::new_default(' '));
                        }
                    }
                }
                _ => continue,
            };
        }

        // checks the size and resize the buffer if required.
        check_size(&application, &mut buffer)?;

        // checks the app stats and draw them in the stat bar.
        draw_stats(&application, &mut buffer, fps)?;

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

fn check_size(application: &Application, buffer: &mut CellBuffer) -> Result<()> {
    let size = application.get_terminal().get_console_size()?;

    if size.width != buffer.size.width || size.height != buffer.size.height {
        buffer.resize(Cell::new(' ', Color::Black, Color::Black), size);
    }

    Ok(())
}

fn draw_stats(application: &Application, buffer: &mut CellBuffer, fps: i32) -> Result<()> {
    let text_background = Cell::new(' ', Color::White, Color::DarkGrey);
    let separator = Cell::new('¯', Color::Grey, Color::Black);
    let window = application.get_window();
    let terminal = application.get_terminal();
    let console_size = terminal.get_console_size()?;
    let window_size = window.get_window_client_size()?;
    let char_size = terminal.get_char_size(window)?;

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

    Ok(())
}