use std::cmp::Ordering;
use std::env;
use std::fmt::Write;
use std::marker::PhantomData;
use std::path::PathBuf;

#[cfg(feature = "libgit")]
use libgit as internal;
#[cfg(not(feature = "libgit"))]
use process as internal;

use crate::colors::Color;
use crate::themes::DefaultColors;
use crate::{Powerline, Style};

use super::Module;

#[cfg(not(feature = "libgit"))]
mod process;

#[cfg(feature = "libgit")]
mod libgit;

pub struct Git<S> {
    scheme: PhantomData<S>,
}

pub trait GitScheme: DefaultColors {
    fn git_remote_bg() -> Color {
        Self::default_bg()
    }
    fn git_remote_fg() -> Color {
        Self::default_fg()
    }
    fn git_staged_bg() -> Color {
        Self::default_bg()
    }
    fn git_staged_fg() -> Color {
        Self::default_fg()
    }
    fn git_notstaged_bg() -> Color {
        Self::default_bg()
    }
    fn git_notstaged_fg() -> Color {
        Self::default_fg()
    }
    fn git_untracked_bg() -> Color {
        Self::default_bg()
    }
    fn git_untracked_fg() -> Color {
        Self::default_fg()
    }
    fn git_conflicted_bg() -> Color {
        Self::default_bg()
    }
    fn git_conflicted_fg() -> Color {
        Self::default_fg()
    }
    fn git_repo_clean_bg() -> Color {
        Self::default_bg()
    }
    fn git_repo_clean_fg() -> Color {
        Self::default_fg()
    }
    fn git_repo_dirty_bg() -> Color {
        Self::default_bg()
    }
    fn git_repo_dirty_fg() -> Color {
        Self::default_fg()
    }

    const NOT_STAGED_SYMBOL: &'static str = PENCIL;
    const STAGED_SYMBOL: &'static str = "+";
    const UNTRACKED_SYMBOL: &'static str = "?";
    const CONFLICTED_SYMBOL: &'static str = FANCY_STAR;
}

impl<S: GitScheme> Default for Git<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: GitScheme> Git<S> {
    pub fn new() -> Git<S> {
        Git {
            scheme: PhantomData,
        }
    }
}

pub struct GitStats {
    pub untracked: u32,
    pub conflicted: u32,
    pub non_staged: u32,
    pub ahead: u32,
    pub behind: u32,
    pub staged: u32,
    pub remote: bool,
    pub branch_name: String,
}

impl GitStats {
    pub fn is_dirty(&self) -> bool {
        (self.untracked + self.conflicted + self.staged + self.non_staged) > 0
    }
}

fn find_git_dir() -> Option<PathBuf> {
    let mut git_dir = env::current_dir().unwrap();
    loop {
        git_dir.push(".git/");

        if git_dir.exists() {
            git_dir.pop();
            return Some(git_dir);
        }
        git_dir.pop();

        if !git_dir.pop() {
            return None;
        }
    }
}

const UP_ARROW: &str = "\u{f062}";
const DOWN_ARROW: &str = "\u{f063}";
const PENCIL: &str = "\u{eae9}";
const FANCY_STAR: &str = "\u{273C}";

const GITHUB_LOGO: &str = "\u{e709}";
const GIT_ICON: &str = "\u{e0a0}";

impl<S: GitScheme> Module for Git<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let git_dir = match find_git_dir() {
            Some(dir) => dir,
            _ => return,
        };

        let stats = internal::run_git(&git_dir);

        let (branch_fg, branch_bg) = if stats.is_dirty() {
            (S::git_repo_dirty_fg(), S::git_repo_dirty_bg())
        } else {
            (S::git_repo_clean_fg(), S::git_repo_clean_bg())
        };

        powerline.add_segment(
            format!("{} {}", GIT_ICON, stats.branch_name),
            Style::simple(branch_fg, branch_bg),
        );

        let add_elem = |powerline: &mut Powerline, count: u32, symbol, fg, bg| match count.cmp(&1) {
            Ordering::Equal | Ordering::Greater => {
                powerline.add_segment(format!("{} {}", count, symbol), Style::simple(fg, bg))
            }
            Ordering::Less => (),
        };

        add_elem(
            powerline,
            stats.non_staged,
            S::NOT_STAGED_SYMBOL,
            S::git_notstaged_fg(),
            S::git_notstaged_bg(),
        );
        add_elem(
            powerline,
            stats.untracked,
            S::UNTRACKED_SYMBOL,
            S::git_untracked_fg(),
            S::git_untracked_bg(),
        );
        add_elem(
            powerline,
            stats.staged,
            S::STAGED_SYMBOL,
            S::git_staged_fg(),
            S::git_staged_bg(),
        );
        add_elem(
            powerline,
            stats.conflicted,
            S::CONFLICTED_SYMBOL,
            S::git_conflicted_fg(),
            S::git_conflicted_bg(),
        );

        if stats.remote {
            let logo_padding = if stats.ahead > 0 || stats.behind > 0 {
                " "
            } else {
                ""
            };
            let mut remote: String = format!("{}{}", GITHUB_LOGO, logo_padding);

            if stats.ahead > 0 {
                let _ = write!(remote, "{}{} ", stats.ahead, UP_ARROW);
            }
            if stats.behind > 0 {
                let _ = write!(remote, "{}{}", stats.behind, DOWN_ARROW);
            }

            powerline.add_segment(
                remote,
                Style::simple(S::git_remote_fg(), S::git_remote_bg()),
            );
        }
    }
}
