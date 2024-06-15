use std::env;
use std::marker::PhantomData;

use crate::colors::white;
use crate::modules::Module;
use crate::{Color, Powerline, Style};

pub struct Cargo<S> {
    scheme: PhantomData<S>,
}

pub trait CargoScheme {
    const CARGO_FG: Color = white();
    const CARGO_BG: Color;

    const ICON: &'static str = "\u{e68b}";
}

impl<S: CargoScheme> Default for Cargo<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: CargoScheme> Cargo<S> {
    pub fn new() -> Cargo<S> {
        Cargo {
            scheme: PhantomData,
        }
    }
}

impl<S: CargoScheme> Module for Cargo<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if let Ok(cwd) = env::current_dir() {
            if cwd.join("Cargo.toml").exists() {
                powerline.add_segment(S::ICON.to_string(), Style::simple(S::CARGO_FG, S::CARGO_BG));
            }
        }
    }
}
