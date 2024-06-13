use std::marker::PhantomData;

use std::time::Instant;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Time<S: TimeScheme> {
    time_format: &'static str,
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
            time_format: "%H:%M:%S",
            scheme: PhantomData,
        }
    }

    pub fn with_time_format(time_format: &'static str) -> Time<S> {
        Time {
            time_format,
            scheme: PhantomData,
        }
    }
}

impl<S: TimeScheme> Module for Time<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        // todo: bring back proper formatting for time
        let now = Instant::now();
        powerline.add_segment(format!("{:?}", now), Style::simple(S::TIME_FG, S::TIME_BG));
    }
}
