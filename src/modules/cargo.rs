use std::env;
use std::marker::PhantomData;

use crate::colors::Color;
use crate::modules::Module;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

pub struct Cargo<S> {
    scheme: PhantomData<S>,
}

pub trait CargoScheme: DefaultColors {
    fn cargo_fg() -> Color {
        Self::default_fg()
    }

    fn cargo_bg() -> Color {
        Self::default_bg()
    }

    fn icon() -> &'static str {
        "\u{e68b}"
    }
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
                powerline.add_segment(
                    S::icon().to_string(),
                    Style::simple(S::cargo_fg(), S::cargo_bg()),
                );
            }
        }
    }
}
