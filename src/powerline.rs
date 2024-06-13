use std::fmt::{Display, Write};

use crate::modules::Module;
use crate::terminal::*;

#[derive(Clone)]
pub struct Style {
    pub fg: FgColor,
    pub bg: BgColor,
    pub sep: Option<Separator>,
    pub sep_fg: FgColor,
}

impl Style {
    pub fn simple(fg: Color, bg: Color) -> Style {
        Style {
            fg: fg.into(),
            bg: bg.into(),
            sep: None, // use the "default" separator
            sep_fg: bg.into(),
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
    ShortAngleBracketRight,
    ZeroWidthSpace,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

// impl Separator {
//     fn direction(&self) -> Direction {
//         match self {
//             Separator::ChevronRight
//             | Separator::RoundRight
//             | Separator::AngleLineRight
//             | Separator::ShortAngleBracketRight => Direction::Right,
//             Separator::ChevronLeft | Separator::RoundLeft | Separator::AngleLineLeft => {
//                 Direction::Left
//             }
//             Separator::ZeroWidthSpace => Direction::None,
//         }
//     }
// }

impl From<Separator> for char {
    fn from(value: Separator) -> Self {
        match value {
            Separator::ChevronRight => '\u{e0b0}',
            Separator::ChevronLeft => '\u{e0b2}',
            Separator::RoundRight => '\u{e0b4}',
            Separator::RoundLeft => '\u{e0b6}',
            Separator::AngleLineRight => '\u{e0b1}',
            Separator::AngleLineLeft => '\u{e0b3}',
            Separator::ShortAngleBracketRight => '\u{276D}',
            Separator::ZeroWidthSpace => '\u{200B}',
        }
    }
}

pub struct Powerline {
    left_buffer: String,
    left_columns: usize, // counting only visible characters...hopefully
    right_buffer: String,
    right_columns: usize, // likewise for the right buffer
    last_style: Option<Style>,
    last_style_right: Option<Style>,
    separator: Separator,
    direction: Direction,
}

impl Default for Powerline {
    fn default() -> Self {
        Self::new()
    }
}

impl Powerline {
    pub fn new() -> Powerline {
        Powerline {
            left_buffer: String::with_capacity(512),
            left_columns: 0,
            right_buffer: String::with_capacity(512),
            right_columns: 0,
            last_style: None,
            last_style_right: None,
            separator: Separator::ChevronRight,
            direction: Direction::Left,
        }
    }

    pub fn set_separator(mut self, separator: Separator) -> Self {
        self.separator = separator;
        self
    }

    #[inline(always)]
    fn write_segment<D: Display>(&mut self, seg: D, style: Style, spaces: bool) {
        // write the last style's separator on the new style's background
        let _ = if let Some(Style { sep_fg, sep, .. }) = self.last_style {
            let sep: char = sep.unwrap_or(self.separator).into();
            self.left_columns += 1;
            write!(self.left_buffer, "{}{}{}", style.bg, sep_fg, sep)
        } else {
            write!(self.left_buffer, "{}", style.bg)
        };

        if self.last_style.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            let _ = write!(self.left_buffer, "{}", style.fg);
        }

        let orig_len = self.left_buffer.len();
        let _ = if spaces {
            write!(self.left_buffer, " {} ", seg)
        } else {
            write!(self.left_buffer, "{}", seg)
        };

        // attempt to account for symbols in the segment by assuming all chars
        // printed are of length 1
        self.left_columns += self.left_buffer[orig_len..].chars().count();

        self.last_style = Some(style)
    }

    fn write_segment_right<D: Display>(&mut self, seg: D, style: Style, spaces: bool) {
        let sep: char = style.sep.unwrap_or(self.separator).into();
        // write the separator directly onto the current background
        let _ = write!(self.right_buffer, "{}{}{}", style.sep_fg, sep, style.bg);
        self.right_columns += 1;

        if self.last_style_right.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            let _ = write!(self.right_buffer, "{}", style.fg);
        }

        let orig_len = self.right_buffer.len();
        let _ = if spaces {
            write!(self.right_buffer, " {} ", seg)
        } else {
            write!(self.right_buffer, "{}", seg)
        };

        // attempt to account for symbols in the segment by assuming all chars
        // printed are of length 1 (so multi-byte chars don't over-inflate the size)
        self.right_columns += self.right_buffer[orig_len..].chars().count();

        self.last_style_right = Some(style)
    }

    pub fn add_segment<D: Display>(&mut self, seg: D, style: Style) {
        match self.direction {
            Direction::Left => self.write_segment(seg, style, true),
            Direction::Right => self.write_segment_right(seg, style, true),
        }
    }

    pub fn add_short_segment<D: Display>(&mut self, seg: D, style: Style) {
        match self.direction {
            Direction::Left => self.write_segment(seg, style, false),
            Direction::Right => self.write_segment_right(seg, style, false),
        }
    }

    pub fn to_right(mut self) -> Self {
        assert_eq!(self.direction, Direction::Left);
        self.close_left_buffer();
        self.direction = Direction::Right;
        self
    }

    pub fn add_module<M: Module>(mut self, mut module: M) -> Self {
        module.append_segments(&mut self);
        self
    }

    pub fn add_padding(mut self, len: usize, bg: Option<Color>) -> Self {
        let padding = vec![" "; len].join("");
        self.left_columns += len;
        match self.direction {
            Direction::Left => {
                self.close_left_buffer();
                match bg {
                    Some(color) => write!(self.left_buffer, "{}{}", BgColor::from(color), padding).unwrap(),
                    None => write!(self.left_buffer, "{}{}", Reset, padding).unwrap(),
                }
            }
            Direction::Right => todo!(),
        }

        self
    }

    pub fn last_style_mut(&mut self) -> Option<&mut Style> {
        self.last_style.as_mut()
    }

    pub fn render(mut self, total_columns: usize) -> String {
        let mut output = String::with_capacity(512);

        // don't print any padding if there's no right prompt
        if let Direction::Left = self.direction {
            // to_right closes out the buffer
            self.close_left_buffer();
            return self.left_buffer;
        }

        // careful not to underflow
        let padding = total_columns
            .checked_sub(self.left_columns)
            .and_then(|cols| cols.checked_sub(self.right_columns))
            .and_then(|cols| cols.checked_sub(1)) // extra padding for safety
            .unwrap_or(0);

        // println!(
        //     "columns: {total_columns}, padding: {padding}, left: {} right: {}",
        //     self.left_columns, self.right_columns
        // );

        let padding = vec![" "; padding].join("");

        let _ = write!(
            output,
            "{}{}{}{}",
            self.left_buffer, padding, self.right_buffer, Reset
        );

        output
    }

    fn close_left_buffer(&mut self) {
        // close out the left buffer with the right separator
        if let Some(Style { sep_fg, sep, .. }) = self.last_style {
            let sep: char = sep.unwrap_or(self.separator).into();
            write!(self.left_buffer, "{}{}{}{}", Reset, sep_fg, sep, Reset).unwrap();
            self.left_columns += 1;
        }
        self.last_style = None;
    }
}
