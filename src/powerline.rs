use crate::config;
use crate::config::{Config, LineSegment, SeparatorStyle};
use std::fmt;
use std::fmt::{Display, Write};
use std::time::Duration;

use crate::modules::{Cwd, Module, Spacer, LastCmdDuration, PythonEnv, Cmd, Git};
use crate::terminal::*;
use crate::themes::CompleteTheme;

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
    Chevron,
    Round,
    AngleLine,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Separator {
    fn for_direction(&self, direction: Direction) -> char {
        match (self, direction) {
            (Separator::Chevron, Direction::Right) => '\u{e0b0}',
            (Separator::Chevron, Direction::Left) => '\u{e0b2}',
            (Separator::Round, Direction::Right) => '\u{e0b4}',
            (Separator::Round, Direction::Left) => '\u{e0b6}',
            (Separator::AngleLine, Direction::Right) => '\u{e0b1}',
            (Separator::AngleLine, Direction::Left) => '\u{e0b3}',
        }
    }
}

impl From<&SeparatorStyle> for Separator {
    fn from(style: &SeparatorStyle) -> Self {
        match style {
            SeparatorStyle::Chevron => Separator::Chevron,
            SeparatorStyle::Round => Separator::Round
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
    last_padding: bool,
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
            separator: Separator::Chevron,
            direction: Direction::Left,
            last_padding: false,
        }
    }

    pub fn from_conf<T: CompleteTheme>(conf: &config::CommandLine) -> Self {
        let mut powerline = Powerline::new();
        powerline.add_conf_modules::<T>(&conf.left);

        if let Some(right_modules) = &conf.right {
            let mut powerline = powerline.to_right();
            powerline.add_conf_modules::<T>(right_modules);
            return powerline;
        }

        powerline
    }

    pub fn set_separator(mut self, separator: Separator) -> Self {
        self.separator = separator;
        self
    }

    #[inline(always)]
    fn write_segment<D: Display>(&mut self, seg: D, style: Style, spaces: bool) -> fmt::Result {
        // write the last style's separator on the new style's background
        if self.last_padding {
            let new_sep = style.sep.unwrap_or(self.separator);
            write!(
                self.left_buffer,
                "{}{}",
                style.sep_fg,
                new_sep.for_direction(Direction::Left)
            )?;
            self.last_padding = false;
        }

        if let Some(Style { sep_fg, sep, .. }) = self.last_style {
            let sep: char = sep
                .unwrap_or(self.separator)
                .for_direction(Direction::Right);
            self.left_columns += 1;
            write!(self.left_buffer, "{}{}{}", style.bg, sep_fg, sep)?;
        } else {
            write!(self.left_buffer, "{}", style.bg)?;
        };

        if self.last_style.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            write!(self.left_buffer, "{}", style.fg)?;
        }

        let orig_len = self.left_buffer.len();
        if spaces {
            write!(self.left_buffer, " {} ", seg)?;
        } else {
            write!(self.left_buffer, "{}", seg)?;
        };

        // attempt to account for symbols in the segment by assuming all chars
        // printed are of length 1
        self.left_columns += self.left_buffer[orig_len..].chars().count();

        self.last_style = Some(style);
        Ok(())
    }

    fn write_segment_right<D: Display>(
        &mut self,
        seg: D,
        style: Style,
        spaces: bool,
    ) -> fmt::Result {
        let sep: char = style
            .sep
            .unwrap_or(self.separator)
            .for_direction(Direction::Left);
        // write the separator directly onto the current background
        write!(self.right_buffer, "{}{}{}", style.sep_fg, sep, style.bg)?;
        self.right_columns += 1;

        if self.last_style_right.as_ref().map(|s| s.sep_fg) != Some(style.fg) {
            write!(self.right_buffer, "{}", style.fg)?;
        }

        let orig_len = self.right_buffer.len();
        if spaces {
            write!(self.right_buffer, " {} ", seg)?;
        } else {
            write!(self.right_buffer, "{}", seg)?;
        };

        // attempt to account for symbols in the segment by assuming all chars
        // printed are of length 1 (so multi-byte chars don't over-inflate the size)
        self.right_columns += self.right_buffer[orig_len..].chars().count();

        self.last_style_right = Some(style);
        Ok(())
    }

    pub fn add_segment<D: Display>(&mut self, seg: D, style: Style) {
        let _ = match self.direction {
            Direction::Left => self.write_segment(seg, style, true),
            Direction::Right => self.write_segment_right(seg, style, true),
        };
    }

    pub fn add_short_segment<D: Display>(&mut self, seg: D, style: Style) {
        let _ = match self.direction {
            Direction::Left => self.write_segment(seg, style, false),
            Direction::Right => self.write_segment_right(seg, style, false),
        };
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

    fn add_conf_modules<T: CompleteTheme>(&mut self, modules: &Vec<LineSegment>) {
        for module in modules {
            match module {
                LineSegment::SmallSpacer => Spacer::<T>::small().append_segments(self),
                LineSegment::LargeSpacer => Spacer::<T>::large().append_segments(self),
                LineSegment::LastCmdDuration { min_run_time } => LastCmdDuration::<T>::new(
                    Duration::from_secs(1),
                    min_run_time.clone(),
                ).append_segments(self),
                LineSegment::Cwd {
                    max_length,
                    wanted_seg_num,
                    resolve_symlinks,
                } => Cwd::<T>::new(*max_length, *wanted_seg_num, *resolve_symlinks).append_segments(self),
                LineSegment::PythonEnv => PythonEnv::<T>::new().append_segments(self),
                LineSegment::Cmd => Cmd::<T>::new("0".into()).append_segments(self),
                LineSegment::Git => Git::<T>::new().append_segments(self),
                _ => todo!(),
            };
        }
    }

    pub fn add_padding(mut self, len: usize) -> Self {
        let padding = vec![" "; len].join("");
        match self.direction {
            Direction::Left => {
                // close out the buffer, write the padding, and leave the next write_segment
                // to handle adding the alternate separator
                self.close_left_buffer();
                self.left_columns += len;
                let _ = write!(self.left_buffer, "{}{}", Reset, padding);
            }
            Direction::Right => {
                // close out the current blob and write the padding
                if let Some(Style { sep, sep_fg, .. }) = self.last_style_right {
                    let sep: char = sep
                        .unwrap_or(self.separator)
                        .for_direction(Direction::Right);
                    write!(
                        self.right_buffer,
                        "{}{}{}{}{}",
                        Reset, sep_fg, sep, Reset, padding
                    )
                        .unwrap();
                    self.right_columns += 1;
                } else {
                    write!(self.right_buffer, "{}", padding).unwrap();
                }
                self.right_columns += len;
                self.last_style = None;
            }
        }

        self.last_padding = true;

        self
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
            let sep: char = sep
                .unwrap_or(self.separator)
                .for_direction(Direction::Right);
            write!(self.left_buffer, "{}{}{}{}", Reset, sep_fg, sep, Reset).unwrap();
            self.left_columns += 1;
        }
        self.last_style = None;
    }
}
