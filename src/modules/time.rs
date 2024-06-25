use std::marker::PhantomData;

use chrono::Local;

use super::Module;
use crate::colors::Color;
use crate::{Powerline, Style};

pub struct Time<S: TimeScheme> {
    time_format: String,
    scheme: PhantomData<S>,
}

pub trait TimeScheme {
    const TIME_BG: Color;
    const TIME_FG: Color;
}

impl<S: TimeScheme> Default for Time<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: TimeScheme> Time<S> {
    pub fn new() -> Time<S> {
        Time {
            time_format: "%H:%M:%S".into(),
            scheme: PhantomData,
        }
    }

    pub fn with_time_format(time_format: String) -> Time<S> {
        Time {
            time_format,
            scheme: PhantomData,
        }
    }
}

impl<S: TimeScheme> Module for Time<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let now = Local::now().format(&self.time_format).to_string();
        powerline.add_segment(now, Style::simple(S::TIME_FG, S::TIME_BG));
    }
}
