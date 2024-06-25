use std::ffi::CString;
use std::marker::PhantomData;

use crate::{Powerline, Style};
use crate::colors::Color;
use crate::themes::DefaultColors;

use super::Module;

pub struct ReadOnly<S>(PhantomData<S>);

pub trait ReadOnlyScheme: DefaultColors {
    fn readonly_fg() -> Color {
        Self::default_fg()
    }
    fn readonly_bg() -> Color {
        Self::default_bg()
    }

    fn readonly_symbol() -> &'static str {
        ""
    }
}

impl<S: ReadOnlyScheme> Default for ReadOnly<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: ReadOnlyScheme> ReadOnly<S> {
    pub fn new() -> ReadOnly<S> {
        ReadOnly(PhantomData)
    }
}

impl<S: ReadOnlyScheme> Module for ReadOnly<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let readonly = unsafe {
            let path = CString::new("./").unwrap();
            libc::access(path.as_ptr(), libc::W_OK) != 0
        };

        if readonly {
            powerline.add_segment(
                S::readonly_symbol(),
                Style::simple(S::readonly_fg(), S::readonly_bg()),
            );
        }
    }
}
