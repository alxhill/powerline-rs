use std::env;
use std::fs::File;
use std::io::read_to_string;
use std::marker::PhantomData;

use crate::colors::Color;
use crate::modules::Module;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

pub struct Nvm<S> {
    scheme: PhantomData<S>,
}

pub trait NvmScheme: DefaultColors {
    fn nvm_fg() -> Color {
        Self::default_fg()
    }

    fn nvm_bg() -> Color {
        Self::default_bg()
    }

    fn nvm_inactive_bg() -> Color {
        Self::default_bg()
    }

    fn icon() -> &'static str {
        "\u{ed0d}"
    }
}

impl<S: NvmScheme> Default for Nvm<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: NvmScheme> Nvm<S> {
    pub fn new() -> Nvm<S> {
        Nvm {
            scheme: PhantomData,
        }
    }
}

impl<S: NvmScheme> Module for Nvm<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let nvm_current_version = env::var("nvm_current_version").ok();

        let nvmrc_version = env::current_dir()
            .and_then(|cwd| File::open(cwd.join(".nvmrc")))
            .and_then(read_to_string)
            .ok();

        match (nvm_current_version, nvmrc_version) {
            // todo: handle the case where active version != .nvmrc
            (Some(version), _) => {
                powerline.add_segment(
                    format!("{} {}", S::icon().to_string(), version),
                    Style::simple(S::nvm_fg(), S::nvm_bg()),
                );
            }
            (None, Some(nvmrc)) => {
                powerline.add_segment(
                    format!("{} {}", S::icon().to_string(), nvmrc),
                    Style::simple(S::nvm_fg(), S::nvm_inactive_bg()),
                );
            }
            _ => {}
        }
    }
}
