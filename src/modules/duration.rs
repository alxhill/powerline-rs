use std::env;
use std::marker::PhantomData;
use chrono::Duration;
use crate::{Color, Powerline, Style};
use crate::modules::Module;
use crate::powerline::Separator;

pub struct LastCmdDuration<S> {
    min_display_time: Duration,
    scheme: PhantomData<S>
}

pub trait LastCmdDurationScheme {
    const TIME_BG: Color;
    const TIME_FG: Color;
    const TIME_ICON: &'static str = "\u{f1acc}"; // time with !
}

impl<S: LastCmdDurationScheme> LastCmdDuration<S> {
    pub fn new(min_duration: Duration) -> LastCmdDuration<S> {
        LastCmdDuration {
            min_display_time: min_duration,
            scheme: PhantomData
        }
    }
}

impl<S: LastCmdDurationScheme> Module for LastCmdDuration<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if let Some(duration) = env::args().nth(2) {
            if let Ok(duration) = str::parse::<i64>(&duration) {
                let cmd_duration = Duration::milliseconds(duration);
                if cmd_duration > self.min_display_time {
                    powerline.add_segment(S::TIME_ICON, Style::custom(S::TIME_FG, S::TIME_BG, Separator::RoundRight));
                }
            }
        }
    }
}