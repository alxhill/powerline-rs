use std::marker::PhantomData;
use std::time::Duration;

use crate::colors::Color;
use crate::modules::Module;
use crate::{Powerline, Style};

pub struct LastCmdDuration<S> {
    min_display_time: Duration,
    cmd_duration: Option<Duration>,
    scheme: PhantomData<S>,
}

pub trait LastCmdDurationScheme {
    const TIME_BG: Color;
    const TIME_FG: Color;
    const TIME_ICON: &'static str = "\u{f1acc}"; // time with !
}

impl<S: LastCmdDurationScheme> LastCmdDuration<S> {
    pub fn new(cmd_duration: Option<Duration>, min_duration: Duration) -> LastCmdDuration<S> {
        LastCmdDuration {
            min_display_time: min_duration,
            cmd_duration,
            scheme: PhantomData,
        }
    }
}

impl<S: LastCmdDurationScheme> Module for LastCmdDuration<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if let Some(cmd_dur) = self.cmd_duration {
            if cmd_dur > self.min_display_time {
                powerline.add_short_segment(
                    format!(" {}{}", nice_duration(cmd_dur), S::TIME_ICON),
                    Style::simple(S::TIME_FG, S::TIME_BG),
                );
            }
        }
    }
}

fn nice_duration(dur: Duration) -> String {
    if dur > Duration::from_secs(60) {
        return format!("{}m{}s", dur.as_secs() / 60, dur.as_secs() % 60);
    }

    if dur > Duration::from_secs(1) {
        return format!("{:.2}s", dur.as_millis() as f32 / 1000f32);
    }

    if dur > Duration::from_millis(1) {
        return format!("{}ms", dur.as_millis());
    }

    format!("{}µs", dur.as_millis())
}
