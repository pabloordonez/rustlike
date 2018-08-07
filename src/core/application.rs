use core::mouse::Mouse;
use core::window::Window;
use core::terminal::Terminal;
use core::events::event_queue::EventQueue;
use core::Result;

pub trait Application {
    fn get_terminal(&self) -> &Terminal;

    fn get_window(&self) -> &Window;

    fn get_mouse(&self) -> &Mouse;

    fn get_event_queue(&self) -> &EventQueue;

    fn get_mut_event_queue(&mut self) -> &mut EventQueue;

    fn listen_events(&mut self) -> Result<()>;
}