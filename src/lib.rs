pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;

pub(crate) mod utils;
mod colors;

pub use crate::powerline::{Powerline, Style};
pub use crate::terminal::Color;
