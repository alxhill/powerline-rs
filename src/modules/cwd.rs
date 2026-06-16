use std::marker::PhantomData;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use std::{env, path};

use crate::colors::Color;
use crate::platform;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

use super::Module;

pub struct Cwd<S: CwdScheme> {
    max_length: usize,
    wanted_seg_num: usize,
    resolve_symlinks: bool,
    scheme: PhantomData<S>,
}

pub trait CwdScheme: DefaultColors {
    fn path_fg() -> Color {
        Self::default_fg()
    }

    fn path_bg_colors() -> Vec<Color>;
}

impl<S: CwdScheme> Cwd<S> {
    pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<S> {
        Cwd {
            max_length,
            wanted_seg_num,
            resolve_symlinks,
            scheme: PhantomData,
        }
    }
}

macro_rules! rainbow_segment {
    ($powerline:ident, $iter_var:ident, $value:expr) => {
        let r_col = S::path_bg_colors()[$iter_var % S::path_bg_colors().len()];
        $powerline.add_short_segment(format!(" {}", $value), Style::simple(S::path_fg(), r_col));
        $iter_var = $iter_var.wrapping_add(1);
    };
}

impl<S: CwdScheme> Module for Cwd<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let current_dir = if self.resolve_symlinks {
            env::current_dir().unwrap()
        } else {
            // $PWD is the shell's logical (symlink-preserving) cwd, but it isn't
            // set on Windows or by every shell, so fall back to the real cwd.
            env::var_os("PWD")
                .map(path::PathBuf::from)
                .unwrap_or_else(|| env::current_dir().unwrap())
        };

        let current_dir = current_dir.to_string_lossy();
        let mut cwd: &str = &current_dir;

        let mut current_bg = 0usize;

        // Sitting at the filesystem root ("/" on Unix) - just show the glyph.
        #[allow(unused_assignments)]
        if cwd == MAIN_SEPARATOR_STR {
            rainbow_segment!(powerline, current_bg, "~");
            return;
        }

        if let Some(home) = platform::home_dir() {
            let home = home.to_string_lossy();
            if cwd.starts_with(home.as_ref()) {
                rainbow_segment!(powerline, current_bg, "~");
                cwd = &cwd[home.len()..]
            }
        }

        let depth = cwd.matches(MAIN_SEPARATOR).count();

        if (cwd.len() > self.max_length) && (depth > self.wanted_seg_num) {
            let left = self.wanted_seg_num / 2;
            let right = self.wanted_seg_num - left;

            let start = cwd.split(MAIN_SEPARATOR).skip(1).take(left);
            let end = cwd.split(MAIN_SEPARATOR).skip(depth - right + 1);

            for val in start {
                rainbow_segment!(powerline, current_bg, val);
            }

            rainbow_segment!(powerline, current_bg, "...");

            for val in end {
                rainbow_segment!(powerline, current_bg, val);
            }
        } else {
            for val in cwd.split(MAIN_SEPARATOR).skip(1) {
                rainbow_segment!(powerline, current_bg, val);
            }
        };
    }
}
