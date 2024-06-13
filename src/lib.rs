pub mod colors;
pub mod config;
pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod themes;

pub(crate) mod utils;

pub use crate::powerline::{Powerline, Style};
pub use crate::terminal::Color;
