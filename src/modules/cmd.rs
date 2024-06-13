use std::marker::PhantomData;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cmd<S: CmdScheme> {
    status: String,
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
            "0" => (user_symbol, S::CMD_PASSED_FG, S::CMD_PASSED_BG),
            non_zero_code => (non_zero_code, S::CMD_FAILED_FG, S::CMD_FAILED_BG),
        };

        powerline.add_short_segment(symbol, Style::simple(fg, bg));
    }
}
