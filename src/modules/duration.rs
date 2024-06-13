use crate::modules::Module;
use crate::{Color, Powerline, Style};
use std::time::Duration;
use std::marker::PhantomData;

pub struct LastCmdDuration<S> {
    min_display_time: Duration,
    cmd_duration: Duration,
    scheme: PhantomData<S>,
}

pub trait LastCmdDurationScheme {
    const TIME_BG: Color;
    const TIME_FG: Color;
    const TIME_ICON: &'static str = "\u{f1acc}"; // time with !
}

impl<S: LastCmdDurationScheme> LastCmdDuration<S> {
    pub fn new(cmd_duration: Duration, min_duration: Duration) -> LastCmdDuration<S> {
        LastCmdDuration {
            min_display_time: min_duration,
            cmd_duration,
            scheme: PhantomData,
        }
    }
}

impl<S: LastCmdDurationScheme> Module for LastCmdDuration<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if self.cmd_duration > self.min_display_time {
            powerline.add_short_segment(
                format!(" {}{}", nice_duration(self.cmd_duration), S::TIME_ICON),
                Style::simple(S::TIME_FG, S::TIME_BG),
            );
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
