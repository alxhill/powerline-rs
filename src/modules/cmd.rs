use std::env;
use std::marker::PhantomData;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cmd<S: CmdScheme> {
    scheme: PhantomData<S>,
}

pub trait CmdScheme {
    const CMD_PASSED_FG: Color;
    const CMD_PASSED_BG: Color;
    const CMD_FAILED_BG: Color;
    const CMD_FAILED_FG: Color;
    const CMD_ROOT_SYMBOL: &'static str = "#";
    const CMD_USER_SYMBOL: &'static str = "$";
}

impl<S: CmdScheme> Default for Cmd<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: CmdScheme> Cmd<S> {
    pub fn new() -> Cmd<S> {
        Cmd {
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
        let status_arg = env::args().nth(1);
        let (symbol, fg, bg) = match status_arg.as_deref() {
            None | Some("0") => (user_symbol, S::CMD_PASSED_FG, S::CMD_PASSED_BG),
            Some(non_zero_code) => (non_zero_code, S::CMD_FAILED_FG, S::CMD_FAILED_BG),
        };

        powerline.add_short_segment(symbol, Style::simple(fg, bg));
    }
}
