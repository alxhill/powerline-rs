use std::fmt::{self, Display, Write};

use crate::modules::Module;
use crate::terminal::*;

#[derive(Clone)]
pub struct Style {
    pub fg: FgColor,
    pub bg: BgColor,
    pub sep: char,
    pub sep_fg: FgColor,
}

impl Style {
    pub fn simple(fg: Color, bg: Color) -> Style {
        Style {
            fg: fg.into(),
            bg: bg.into(),
            sep: '\u{E0B0}',
            sep_fg: bg.into(),
        }
    }

    pub fn special(fg: Color, bg: Color, sep: char, sep_fg: Color) -> Style {
        Style {
            fg: fg.into(),
            bg: bg.into(),
            sep,
            sep_fg: sep_fg.into(),
        }
    }

    pub fn custom(fg: Color, bg: Color, separator: Separator) -> Style {
        Style {
            fg: fg.into(),
            bg: bg.into(),
            sep: separator.into(),
            sep_fg: bg.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Separator {
    ChevronRight,
    ChevronLeft,
    RoundRight,
    RoundLeft,
    AngleLineRight,
    AngleLineLeft,
    Custom(char),
}

impl From<Separator> for char {
    fn from(value: Separator) -> Self {
        match value {
            Separator::ChevronRight => '\u{e0b0}',
            Separator::ChevronLeft => '\u{e0b2}',
            Separator::RoundRight => '\u{e0b4}',
            Separator::RoundLeft => '\u{e0b6}',
            Separator::AngleLineRight => '\u{e0b1}',
            Separator::AngleLineLeft => '\u{e0b3}',
            Separator::Custom(c) => c,
        }
    }
}

pub struct Powerline {
    buffer: String,
    last_style: Option<Style>,
}

impl Default for Powerline {
    fn default() -> Self {
        Self::new()
    }
}

impl Powerline {
    pub fn new() -> Powerline {
        Powerline {
            buffer: String::with_capacity(512),
            last_style: None,
        }
    }

    #[inline(always)]
    fn write_segment<D: Display>(&mut self, seg: D, style: Style, spaces: bool) {
        // write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, next.bg, seg.sep_col, seg.sep)?;

        let _ = if let Some(Style { sep_fg, sep, .. }) = self.last_style {
            write!(self.buffer, "{}{}{}", style.bg, sep_fg, sep)
        } else {
            write!(self.buffer, "{}", style.bg)
        };

        if self.last_style.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            let _ = write!(self.buffer, "{}", style.fg);
        }

        let _ = if spaces {
            write!(self.buffer, " {} ", seg)
        } else {
            write!(self.buffer, "{}", seg)
        };

        self.last_style = Some(style)
    }

    pub fn add_segment<D: Display>(&mut self, seg: D, style: Style) {
        self.write_segment(seg, style, true)
    }

    pub fn add_short_segment<D: Display>(&mut self, seg: D, style: Style) {
        self.write_segment(seg, style, false)
    }

    pub fn add_module<M: Module>(&mut self, mut module: M) {
        module.append_segments(self)
    }

    pub fn last_style_mut(&mut self) -> Option<&mut Style> {
        self.last_style.as_mut()
    }
}

impl fmt::Display for Powerline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.last_style {
            Some(Style { sep_fg, sep, .. }) => {
                write!(f, "{}{}{}{}{}", self.buffer, Reset, sep_fg, sep, Reset)
            }
            None => Ok(()),
        }
    }
}
