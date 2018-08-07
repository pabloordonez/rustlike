use std::result;

pub type Result<T> = result::Result<T, &'static str>;

pub mod cell;
pub mod cell_buffer;
pub mod color;
pub mod mouse;
pub mod point_2d;
pub mod size_2d;
pub mod terminal;
pub mod window;
pub mod events;
pub mod application;