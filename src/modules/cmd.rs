use std::marker::PhantomData;

use crate::colors::Color;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

use super::Module;

pub struct Cmd<S: CmdScheme> {
    status: String,
    scheme: PhantomData<S>,
}

pub trait CmdScheme: DefaultColors {
    const CMD_ROOT_SYMBOL: &'static str = "#";
    const CMD_USER_SYMBOL: &'static str = "$";

    fn cmd_passed_fg() -> Color {
        Self::default_fg()
    }

    fn cmd_passed_bg() -> Color {
        Self::default_bg()
    }

    fn cmd_failed_bg() -> Color {
        Self::default_bg()
    }

    fn cmd_failed_fg() -> Color {
        Self::default_fg()
    }

    fn cmd_root_symbol() -> &'static str {
        "#"
    }

    fn cmd_user_symbol() -> &'static str {
        "$"
    }
}

impl<S: CmdScheme> Cmd<S> {
    pub fn new(status: &str) -> Cmd<S> {
        Cmd {
            status: status.into(),
            scheme: PhantomData,
        }
    }
}

impl<S: CmdScheme> Module for Cmd<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let user_symbol = if users::get_current_uid() == 0 {
            S::CMD_ROOT_SYMBOL
        } else {
            S::CMD_USER_SYMBOL
        };
        let (symbol, fg, bg) = match self.status.as_ref() {
            "0" => (user_symbol, S::cmd_passed_fg(), S::cmd_passed_bg()),
            non_zero_code => (non_zero_code, S::cmd_failed_fg(), S::cmd_failed_bg()),
        };

        powerline.add_short_segment(symbol, Style::simple(fg, bg));
    }
}
