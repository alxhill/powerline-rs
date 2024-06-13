use std::marker::PhantomData;
use std::{env, path};

use super::Module;
use crate::colors::white;
use crate::{Color, Powerline, Style};

pub struct Cwd<const N: usize, S: CwdScheme<N>> {
    max_length: usize,
    wanted_seg_num: usize,
    resolve_symlinks: bool,
    scheme: PhantomData<S>,
}

pub trait CwdScheme<const N: usize> {
    const PATH_FG: Color = white();
    const PATH_BG_COLORS: [Color; N];
}

impl<const N: usize, S: CwdScheme<N>> Cwd<N, S> {
    pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<N, S> {
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
        let r_col = S::PATH_BG_COLORS[$iter_var % S::PATH_BG_COLORS.len()];
        $powerline.add_short_segment(format!(" {}", $value), Style::simple(S::PATH_FG, r_col));
        $iter_var = $iter_var.wrapping_add(1);
    };
}

impl<const N: usize, S: CwdScheme<N>> Module for Cwd<N, S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let current_dir = if self.resolve_symlinks {
            env::current_dir().unwrap()
        } else {
            path::PathBuf::from(env::var("PWD").unwrap())
        };

        let mut cwd = current_dir.to_str().unwrap();

        let mut current_bg = 0usize;

        #[allow(unused_assignments)]
        if cwd == "/" {
            rainbow_segment!(powerline, current_bg, "~");
            return;
        }

        if let Ok(home_str) = env::var("HOME") {
            if cwd.starts_with(&home_str) {
                rainbow_segment!(powerline, current_bg, "~");
                cwd = &cwd[home_str.len()..]
            }
        }

        let depth = cwd.matches('/').count();

        if (cwd.len() > self.max_length) && (depth > self.wanted_seg_num) {
            let left = self.wanted_seg_num / 2;
            let right = self.wanted_seg_num - left;

            let start = cwd.split('/').skip(1).take(left);
            let end = cwd.split('/').skip(depth - right + 1);

            for val in start {
                rainbow_segment!(powerline, current_bg, val);
            }

            rainbow_segment!(powerline, current_bg, "...");

            for val in end {
                rainbow_segment!(powerline, current_bg, val);
            }
        } else {
            for val in cwd.split('/').skip(1) {
                rainbow_segment!(powerline, current_bg, val);
            }
        };
    }
}
