use std::result;

pub type Result<T> = result::Result<T, &'static str>;

pub mod application;
pub mod drawing;
pub mod events;
pub mod input;
pub mod mouse;
pub mod terminal;
pub mod window;
